#[repr(C)]
struct Ext4ExtentTail {
    checksum: u32,
}

#[repr(C)]
struct Ext4Extent {
    block: u32,
    len: u16,
    start_hi: u16,
    start_lo: u32,
}

#[repr(C)]
struct Ext4ExtentIdx {
    block: u32,
    leaf_lo: u32,
    leaf_hi: u16,
    unused: u16,
}

#[repr(C)]
struct Ext4ExtentHeader {
    magic: u16,
    entries: u16,
    max: u16,
    depth: u16,
    generation: u32,
}
