pub(super) struct Fat32SuperBlock {
    first_data_sector: u32,
    fat_count: u32,
    data_sector_count: u32,
    bytes_per_sector: u32,
    sectors_per_cluster: u32,
    reserved_sector_count: u32,
    hidden_sector_count: u32,
    total_sector_count: u32,
    sectors_per_fat: u32,
    data_cluster_count: u32,
    bytes_per_cluster: u32,
    root_cluster: u32,
    last_seen_free_cluster: u32,
    search_cluster_hint: u32,
}



