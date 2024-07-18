#[repr(C, packed)]
pub struct Fat32BootParam {
    jmp_boot: [u8; 3],
    oem_name: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sector_count: u16,
    fat_count: u8,
    root_entry_count: u16,
    total_sectors_16: u16,
    media_type: u8,
    fat_size_16: u16,
    sectors_per_track: u16,
    head_count: u16,
    hidden_sector_count: u32,
    total_sectors_32: u32,
    fat_size_32: u32,
    ext_flags: u16,
    fs_version: u16,
    root_cluster: u32,
    fs_info: u16,
    backup_boot_sector: u16,
    reserved: [u8; 12],
    drive_number: u8,
    reserved1: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    fs_type: [u8; 8],
    boot_code: [u8; 420],
    boot_sector_signature: u16,
}

#[repr(C, packed(512))]
pub struct Fat32ExtendedBootRecord {
    fat32_drive_number: u8,
    fat32_reserved: u8,
    fat32_signature: u8,
    fat32_volume_id: u32,
    fat32_volume_label: [u8; 11],
    fat32_fs_type: [u8; 8],
    fat32_boot_code: [u8; 448],
    fat32_boot_signature: u16,
}

#[repr(C, packed(512))]
pub struct Fat32FsInfo {
    fat32_lead_signature: u32,
    fat32_reserved: [u8; 480],
    fat32_signature: u32,
    fat32_last_free_cluster: u32,
    fat32_cluster_hint: u32,
    fat32_reserved1: [u32; 3],
    fat32_trail_signature: u32,
}

