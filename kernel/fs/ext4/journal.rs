#[repr(C)]
struct Ext4JournalSuperBlock {
    header: Ext4JournalHeader,
    blocksize: u32,
    block_count: u32,
    first_block_info: u32,
    first_transaction: u32,
    first_block: u32,
    errno: u32,
    feature_req: u32,
    feature_opt: u32,
    feature_rdonly: u32,
    uuid: [u32; 4],
    fs_count: u32,
    max_blocks_per_transaction: u32,
    max_data_blocks_per_transaction: u32,
    csum_algo: u8,
    padding1: [u8; 3],
    padding2: [u8; 168],
    sb_csum: u32,
    fs_uuid: [u8; 768],
}

#[repr(C)]
struct Ext4JournalHeader {
    magic: u32,
    blocktype: u32,
    journal_transaction: u32,
}
