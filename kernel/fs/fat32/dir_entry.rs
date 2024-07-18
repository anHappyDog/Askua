#[repr(C, packed)]
pub struct Fat32DirEntry {
    name: [u8; 8],
    ext: [u8; 3],
    attr: u8,
    reserved: u8,
    create_time_ms: u8,
    create_time: u16,
    create_date: u16,
    last_access_date: u16,
    first_cluster_high: u16,
    write_time: u16,
    write_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

#[repr(C, packed)]
pub struct Fat32DirEntryLong {
    order: u8,
    name1: [u16; 5],
    attr: u8,
    type_: u8,
    checksum: u8,
    name2: [u16; 6],
    first_cluster_low: u16,
    name3: [u16; 2],
}
