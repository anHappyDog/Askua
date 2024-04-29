#ifndef __FAT_H__
#define __FAT_H__
#include <types.h>

struct fat32_bpb {
  u8 fat32_jmp_boot[3];
  u8 fat32_oem_name[8];
  u16 fat32_bytes_per_sector;
  u8 fat32_sectors_per_cluster;
  u16 fat32_reserved_sectors;
  u8 fat32_fat_count;
  u16 fat32_root_entries;
  u16 fat32_total_sectors;
  u8 fat32_media_type;
  u16 fat32_sectors_per_fat;
  u16 fat32_sectors_per_track;
  u16 fat32_heads;
  u32 fat32_hidden_sectors;
  u32 fat32_total_sectors_large;
} __attribute__((packed));

struct fat32_fs_info {
  u32 fat32_lead_signature;
  u8 fat32_reserved[480];
  u32 fat32_signature;
  u32 fat32_last_free_cluster;
  u32 fat32_cluster_hint;
  u32 fat32_reserved1[3];
  u32 fat32_trail_signature;
} __attribute__((packed, aligned(512)));

struct fat16_extended_boot_record {
  u8 fat16_drive_number;
  u8 fat16_reserved;
  u8 fat16_signature;
  u32 fat16_volume_id;
  u8 fat16_volume_label[11];
  u8 fat16_fs_type[8];
  u8 fat16_boot_code[448];
  u16 fat16_boot_signature;
};

struct fat32_extended_boot_record {
  u8 fat32_drive_number;
  u8 fat32_reserved;
  u8 fat32_signature;
  u32 fat32_volume_id;
  u8 fat32_volume_label[11];
  u8 fat32_fs_type[8];
  u8 fat32_boot_code[448];
  u16 fat32_boot_signature;
} __attribute__((packed, aligned(512)));

#endif // __FAT_H__