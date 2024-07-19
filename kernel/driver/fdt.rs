use alloc::boxed::Box;
use alloc::{collections::BTreeMap, sync::Arc};
use core::error::Error;
use core::fmt::Display;
use core::str::from_utf8;
use elf::header;

use crate::printk;

/// Flattened Device Tree
/// shoudl contain these regions:
///
/// - struct fdt_header
/// (free space)
/// - memory  reservation block
/// (free space)
/// - struct block
/// (free space)
/// - strings block
/// (free space)

/// Flattened Device Tree Header
/// - magic : should contain 0xd00dfeed (big endian)
/// - totalsize : total size of the DT block
/// - off_dt_struct : offset to the structure block,start from the beginning of the header
/// - off_dt_strings : offset to the strings block, start from the beginning of the header
/// - off_mem_rsvmap : offset to the memory reserve map block, start from the beginning of the header
/// - version : version of the block
/// - last_cmp_version : last compatible version
/// - boot_cpuid_phys : physical CPU ID of the boot CPU
/// - size_dt_strings : size of the strings block
/// - size_dt_struct : size of the structure block

pub struct FdtTree<'a> {
    header: FdtHeader,
    root: Arc<FdtNode<'a>>,
    data: &'a [u8],
}

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_REVERSED_MAGIC: u32 = 0x0;

pub enum FdtElem<'a> {
    Node(Arc<FdtNode<'a>>),
    Property(FdtProperty<'a>),
}

pub struct FdtNode<'a> {
    elems: BTreeMap<&'a [u8], FdtElem<'a>>,
    name: &'a [u8],
}

fn skip_u32_be_nop(data: &[u8], index: &mut usize) -> u32 {
    loop {
        let num = get_u32_from_be_data(data, index);
        if num == FDT_NOP || num == 0 {
            *index += 4;
            continue;
        }
        return num;
    }
}

fn skip_u32_le_nop(data: &[u8], index: &mut usize) -> u32 {
    loop {
        let num = get_u32_from_le_data(data, index);
        if num == FDT_NOP {
            *index += 4;
            continue;
        }
        return num;
    }
}

fn get_elem_name_slice<'a>(data: &'a [u8], index: &mut usize) -> &'a [u8] {
    if data.len() <= *index {
        panic!("the data is too short for get the strname.");
    }
    let end = data[*index..]
        .iter()
        .position(|&x| x == 0)
        .map_or(data.len(), |pos| *index + pos);
    let start = *index;
    *index = end + 1;
    if *index % 4 != 0 {
        *index += 4 - (*index % 4);
    }
    &data[start..end]
}

impl<'a> FdtNode<'a> {
    pub fn from_be_bytes(
        data: &'a [u8],
        index: &mut usize,
        offset_string: usize,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let res = skip_u32_be_nop(data, index);
        if res != FDT_BEGIN_NODE {
            return Err(Box::new(FdtNotNodeError));
        }
        *index += 4;
        let name = get_elem_name_slice(data, index);
        let mut map = BTreeMap::<&[u8], FdtElem>::new();
        loop {
            let res = skip_u32_be_nop(data, index);
            match res {
                FDT_BEGIN_NODE => {
                    let node = FdtNode::from_be_bytes(data, index, offset_string)?;
                    map.insert(node.name, FdtElem::Node(node));
                }
                FDT_PROP => {
                    let prop = FdtProperty::from_be_bytes(data, index, offset_string)?;
                    map.insert(prop.name, FdtElem::Property(prop));
                }
                FDT_END_NODE => {
                    *index += 4;
                    break;
                }
                _ => {
                    return Err(Box::new(FdtNotNodeError));
                }
            }
        }
        Ok(Arc::new(Self { elems: map, name }))
    }
    pub fn from_le_bytes(
        data: &'a [u8],
        index: &mut usize,
        offset_string: usize,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let res = skip_u32_le_nop(data, index);
        if res != FDT_BEGIN_NODE {
            return Err(Box::new(FdtNotNodeError));
        }
        *index += 4;
        let name = get_elem_name_slice(data, index);
        printk!("get node: {}\n", from_utf8(name).unwrap());

        let mut map = BTreeMap::<&[u8], FdtElem>::new();
        loop {
            let res = skip_u32_le_nop(data, index);
            match res {
                FDT_BEGIN_NODE => {
                    let node = FdtNode::from_le_bytes(data, index, offset_string)?;
                    map.insert(node.name, FdtElem::Node(node));
                }
                FDT_PROP => {
                    let prop = FdtProperty::from_le_bytes(data, index, offset_string)?;
                    map.insert(prop.name, FdtElem::Property(prop));
                }
                FDT_END_NODE => {
                    *index += 4;
                    break;
                }
                _ => {
                    return Err(Box::new(FdtNotNodeError));
                }
            }
        }
        Ok(Arc::new(Self { elems: map, name }))
    }
}

impl<'a> FdtTree<'a> {
    pub fn from_bytes(ptr: *const u8) -> Result<Self, Box<dyn Error>> {
        let mut index: usize = 0;
        let data = unsafe { core::slice::from_raw_parts(ptr, 40) as &'static [u8] };
        let magic = get_u32_from_be_data(&data, &mut index);
        let (header, root) = if magic == FDT_MAGIC {
            let header = FdtHeader::from_be_bytes(data)?;
            let data = unsafe {
                core::slice::from_raw_parts(ptr, header.totalsize as usize) as &'static [u8]
            };
            index = header.off_dt_struct as usize;
            let root = FdtNode::from_be_bytes(data, &mut index, header.off_dt_strings as usize)?;
            (header, root)
        } else if magic == FDT_REVERSED_MAGIC {
            let header = FdtHeader::from_le_bytes(data)?;
            let data = unsafe {
                core::slice::from_raw_parts(ptr, header.totalsize as usize) as &'static [u8]
            };
            index = header.off_dt_struct as usize;
            let root = FdtNode::from_le_bytes(data, &mut index, header.off_dt_strings as usize)?;
            (header, root)
        } else {
            return Err(Box::new(FdtMagicWrongError));
        };
        Ok(Self { header, root, data })
    }
}

pub struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

impl Display for FdtHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "FdtHeader {{ magic: 0x{:x}, totalsize: 0x{:x}, off_dt_struct: 0x{:x}, off_dt_strings: 0x{:x}, off_mem_rsvmap: 0x{:x}, version: 0x{:x}, last_comp_version: 0x{:x}, boot_cpuid_phys: 0x{:x}, size_dt_strings: 0x{:x}, size_dt_struct: 0x{:x} }}",
            self.magic,
            self.totalsize,
            self.off_dt_struct,
            self.off_dt_strings,
            self.off_mem_rsvmap,
            self.version,
            self.last_comp_version,
            self.boot_cpuid_phys,
            self.size_dt_strings,
            self.size_dt_struct
        )
    }
}

impl FdtHeader {
    pub fn from_be_bytes(data: &[u8]) -> Result<Self, Box<dyn Error>> {
        if data.len() < size_of::<Self>() {
            return Err(Box::new(FdtDataShortError));
        }
        let magic = u32::from_be_bytes(data[0..4].try_into().unwrap());

        if magic == FDT_REVERSED_MAGIC {
            return Err(Box::new(FdtEndianWrongError));
        }

        if magic != FDT_MAGIC {
            return Err(Box::new(FdtMagicWrongError));
        }

        Ok(Self {
            magic: u32::from_be_bytes(data[0..4].try_into().unwrap()),
            totalsize: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            off_dt_struct: u32::from_be_bytes(data[8..12].try_into().unwrap()),
            off_dt_strings: u32::from_be_bytes(data[12..16].try_into().unwrap()),
            off_mem_rsvmap: u32::from_be_bytes(data[16..20].try_into().unwrap()),
            version: u32::from_be_bytes(data[20..24].try_into().unwrap()),
            last_comp_version: u32::from_be_bytes(data[24..28].try_into().unwrap()),
            boot_cpuid_phys: u32::from_be_bytes(data[28..32].try_into().unwrap()),
            size_dt_strings: u32::from_be_bytes(data[32..36].try_into().unwrap()),
            size_dt_struct: u32::from_be_bytes(data[36..40].try_into().unwrap()),
        })
    }
    pub fn from_le_bytes(data: &[u8]) -> Result<Self, Box<dyn Error>> {
        if data.len() < size_of::<Self>() {
            return Err(Box::new(FdtDataShortError));
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());

        if magic == FDT_REVERSED_MAGIC {
            return Err(Box::new(FdtEndianWrongError));
        }

        if magic != FDT_MAGIC {
            return Err(Box::new(FdtMagicWrongError));
        }

        Ok(Self {
            magic: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            totalsize: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            off_dt_struct: u32::from_le_bytes(data[8..12].try_into().unwrap()),
            off_dt_strings: u32::from_le_bytes(data[12..16].try_into().unwrap()),
            off_mem_rsvmap: u32::from_le_bytes(data[16..20].try_into().unwrap()),
            version: u32::from_le_bytes(data[20..24].try_into().unwrap()),
            last_comp_version: u32::from_le_bytes(data[24..28].try_into().unwrap()),
            boot_cpuid_phys: u32::from_le_bytes(data[28..32].try_into().unwrap()),
            size_dt_strings: u32::from_le_bytes(data[32..36].try_into().unwrap()),
            size_dt_struct: u32::from_le_bytes(data[36..40].try_into().unwrap()),
        })
    }
}

pub struct FdtReserveEntry {
    address: u64,
    size: u64,
}

pub struct FdtProperty<'a> {
    name: &'a [u8],
    data: &'a [u8],
}

fn get_u32_from_be_data(data: &[u8], index: &mut usize) -> u32 {
    if data.len() < 4 + *index {
        panic!("data too short for parsing an u32.");
    }
    let res = u32::from_be_bytes(data[*index..*index + 4].try_into().unwrap());
    res
}

fn get_u32_from_le_data(data: &[u8], index: &mut usize) -> u32 {
    if data.len() < 4 + *index {
        panic!("data too short for parsing an u32.");
    }
    let res = u32::from_le_bytes(data[*index..*index + 4].try_into().unwrap());
    res
}

fn get_name_slice<'a>(data: &'a [u8], index: usize) -> &'a [u8] {
    if data.len() <= index {
        panic!("the data is too short for get the strname.");
    }
    let end = data[index..]
        .iter()
        .position(|&x| x == 0)
        .map_or(data.len(), |pos| index + pos);
    &data[index..end]
}

impl<'a> FdtProperty<'a> {
    pub fn from_be_bytes(
        data: &'a [u8],
        index: &mut usize,
        off_string_block: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let mut data_start = 0;
        let res = skip_u32_be_nop(data, index);
        if res != FDT_PROP {
            return Err(Box::new(FdtNotPropertyError));
        }
        *index += 4;
        let value_len = get_u32_from_be_data(data, index);
        *index += 4;
        let nameoff = get_u32_from_be_data(data, index);
        *index += 4;
        let value = &data[*index..(*index + value_len as usize)];
        *index += value_len as usize;
        if value_len % 4 != 0 {
            *index += 4 - (value_len % 4) as usize;
        }
        Ok(Self {
            name: &get_name_slice(data, off_string_block + nameoff as usize),
            data: &value,
        })
    }
    pub fn from_le_bytes(
        data: &'a [u8],
        index: &mut usize,
        off_string_block: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let mut data_start = 0;
        let res = get_u32_from_le_data(data, index);
        if res != FDT_PROP {
            return Err(Box::new(FdtNotPropertyError));
        }
        *index += 4;
        let value_len = get_u32_from_le_data(data, index);
        *index += 4;
        let nameoff = get_u32_from_le_data(data, index);
        let value = &data[*index..(*index + value_len as usize)];
        *index += value_len as usize;
        if value_len % 4 != 0 {
            *index += 4 - (value_len % 4) as usize;
        }
        Ok(Self {
            name: &get_name_slice(data, off_string_block + nameoff as usize),
            data: &value,
        })
    }
}

const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

#[derive(Debug)]
pub struct FdtEndianWrongError;

impl Display for FdtEndianWrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "fdt data endian is wrong.")
    }
}

impl Error for FdtEndianWrongError {}

#[derive(Debug)]
pub struct FdtMagicWrongError;

impl Display for FdtMagicWrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "fdt magic is wrong.")
    }
}

impl Error for FdtMagicWrongError {}

#[derive(Debug)]
pub struct FdtDataShortError;

impl Display for FdtDataShortError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "fdt data is too short for parse")
    }
}

impl Error for FdtDataShortError {}

#[derive(Debug)]
pub struct FdtNotPropertyError;

impl Display for FdtNotPropertyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fdt not property.")
    }
}

impl Error for FdtNotPropertyError {}

#[derive(Debug)]
pub struct FdtNotNodeError;

impl Display for FdtNotNodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, " FdtNotNodeError.")
    }
}

impl Error for FdtNotNodeError {}
