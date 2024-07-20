

struct Ext2Dirent {
    inode: u32,
    rec_len: u16,
    name_len: u8,
    file_type: u8,
    name: [u8; 255],
}