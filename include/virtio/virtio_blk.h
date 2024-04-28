#ifndef __VIRTIO_BLK_H__
#define __VIRTIO_BLK_H__
#include <types.h>
#include <virtio/virtio.h>

//! acutally these metadata should be gotten from the config area.
//! we simply hardcore them here.

#define VIRTIO_BLK_ADDR 0x10008000

#define SECTOR_SIZE 512

#define VIRTIO_MMIO_MAGIC 0x74726976
#define VIRTIO_MMIO_VERSION 0x2

#define VIRTIO_BLK_F_SIZE_MAX 0x1
#define VIRITO_BLK_F_SEG_MAX 0x2
#define VIRTIO_BLK_F_GEOMETRY 0x4
#define VIRTIO_BLK_F_RO 0x5
#define VIRTIO_BLK_F_BLK_SIZE 0x6
#define VIRTIO_BLK_F_FLUSH 0x9
#define VIRTIO_BLK_F_TOPOLOGY 0xa
#define VIRTIO_BLK_F_CONFIG_WCE 0xb
#define VIRTIO_BLK_F_MQ 0xc
#define VIRTIO_BLK_F_DISCARD 0xd
#define VIRTIO_BLK_F_WRITE_ZEROES 0xe
#define VIRTIO_BLK_F_LIFETIME 0xf
#define VIRTIO_BLK_F_SECURE_ERASE 0x10

#define VIRTIO_BLK_DEVICE_ID 2
#define VIRTIO_BLK_VIRTQ_NUM 1

#define MMIO_MAGIC_OFFST 0x0
#define MMIO_VERSION_OFFST 0x4
#define MMIO_DEVICE_ID_OFFST 0x8
#define MMIO_VENDOR_ID_OFFST 0xc
#define MMIO_DEVICE_FEATURES_OFFST 0x10
#define MMIO_DEVICE_FEATURES_SEL_OFFST 0x14
#define MMIO_DRIVER_FEATURES_OFFST 0x20
#define MMIO_DRIVER_FEATURES_SEL_OFFST 0x24
#define MMIO_QUEUE_SEL_OFFST 0x30
#define MMIO_QUEUE_NUM_MAX_OFFST 0x34
#define MMIO_QUEUE_NUM_OFFST 0x38
#define MMIO_QUEUE_READY_OFFST 0x44
#define MMIO_QUEUE_NOTIFY_OFFST 0x50
#define MMIO_INTERRUPT_STATUS_OFFST 0x60
#define MMIO_INTERRUPT_ACK_OFFST 0x64
#define MMIO_STATUS_OFFST 0x70
#define MMIO_QUEUE_DESC_LOW_OFFST 0x80
#define MMIO_QUEUE_DESC_HIGH_OFFST 0x84
#define MMIO_QUEUE_DRIVER_LOW_OFFST 0x90
#define MMIO_QUEUE_DRIVER_HIGH_OFFST 0x94
#define MMIO_QUEUE_DEVICE_LOW_OFFST 0xa0
#define MMIO_QUEUE_DEVICE_HIGH_OFFST 0xa4
#define MMIO_SHMSEL_OFFST 0xac
#define MMIO_SHMLEN_LOW_OFFST 0xb0
#define MMIO_SHMLEN_HIGH_OFFST 0xb4
#define MMIO_SHMBASE_LOW_OFFST 0xb8
#define MMIO_SHMBASE_HIGH_OFFST 0xbc
#define MMIO_QUEUE_RST_OFFST 0xc0
#define MMIO_CONFIG_GEN_OFFST 0xfc
#define MMIO_CONFIG_OFFST 0x100

#define READ_MMIO_REG(base, offset, type) (*(volatile type *)(base + offset))
#define WRITE_MMIO_REG(base, offset, type, value)                              \
  ({ *(volatile size_t *)(base + offset) = value; })

struct virtio_blk_config {
  le64 capacity;
  le32 size_max;
  le32 seg_max;
  struct virtio_blk_geometry {
    le16 cylinders;
    u8 heads;
    u8 sectors;
  } geometry;
  le32 blk_size;
  struct virtio_blk_topology {
    u8 physical_block_exp;
    u8 alignment_offset;
    le16 min_io_size;
    le32 opt_io_size;
  } topology;
  u8 writeback;
  u8 unused0;
  u16 num_queues;
  le32 max_discard_sectors;
  le32 max_discard_seg;
  le32 discard_sector_alignment;
  le32 max_write_zeroes_sectors;
  le32 max_write_zeroes_seg;
  u8 write_zeroes_may_unmap;
  u8 unused1[3];
  le32 max_secure_erase_sectors;
  le32 max_secure_erase_seg;
  le32 secure_erase_sector_alignment;
};

struct virtio_blk_req {
  le32 type;
  le32 reserved;
  le64 sector;
  u8 *data;
  u8 status;
};

#define VIRTIO_BLK_T_IN 0
#define VIRTIO_BLK_T_OUT 1
#define VIRTIO_BLK_T_FLUSH 4
#define VIRTIO_BLK_T_GET_ID 8
#define VIRTIO_BLK_T_GET_LIFETIME 10
#define VIRTIO_BLK_T_DISCARD 11
#define VIRTIO_BLK_T_WRITE_ZEROES 13
#define VIRTIO_BLK_T_SECURE_ERASE 14

struct virtio_blk_discard_write_zeroes {
  le64 sector;
  le32 num_sectors;
  struct {
    le32 unmap : 1;
    le32 reserved : 31;
  } flags;
};

struct virtio_blk_lifetime {
  le16 pre_eol_info;
  le16 device_lifetime_est_typ_a;
  le16 device_lifetime_est_typ_b;
};

#define VIRTIO_BLK_PRE_EOL_INFO_UNDEFINED 0
#define VIRTIO_BLK_PRE_EOL_INFO_NORMAL 1
#define VIRTIO_BLK_PRE_EOL_INFO_WARNING 2
#define VIRTIO_BLK_PRE_EOL_INFO_URGENT 3

#define VIRTIO_BLK_S_OK 0
#define VIRTIO_BLK_S_IOERR 1
#define VIRTIO_BLK_S_UNSUPP 2

void virtio_blk_init(size_t base);

void virtio_blk_read_sectors(size_t base, size_t sector, size_t count,
                             void *buf);
void virtio_blk_write_sectors(size_t base, size_t sector, size_t count,
                              void *buf);

#endif // __VIRTIO_BLK_H__