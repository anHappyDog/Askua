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
struct FdtHeader {
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

struct FdtReserveEntry {
    address: u64,
    size: u64,
}

