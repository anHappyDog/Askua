
#[repr(C)]
struct Ext4XattrHeader {
    magic : u32,
    refcount : u32,
    blocks : u32,
    hash : u32,
    checksum : u32,
    reserved : [u32; 3],

}

#[repr(C)]
struct Ext4XattrIbodyHeader {
    magic : u32
}

#[repr(C)]
struct Ext4XattrEntry {
    name_len : u8,
    name_index : u8,
    value_offs : u16,
    value_block : u32,
    value_size : u32,
    hash : u32,
}

#[repr(C)]
struct Ext4XattrPrefix {
    prefix: usize,
    name_index: u8,
}
