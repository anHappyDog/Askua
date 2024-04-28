#ifndef __VIRTIO_H__
#define __VIRTIO_H__
#include <types.h>

typedef uint64_t le64;
typedef uint32_t le32;
typedef uint16_t le16;
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

#define ACKNOWLEDGE 0x1
#define DRIVER 0x2
#define FAILED 0x80
#define FEATURES_OK 0x8
#define DRIVER_OK 0x4
#define DEVICE_NEEDS_RESET 0x40

#define VIRTIO_F_NOTIFICATION_DATA 38
#define VIRTIO_F_NOTIF_CONFIG_DATA 39
#define VIRTIO_F_ORDER_PLATFORM 36
#define VIRTIO_F_IN_ORDER 35
#define VIRTIO_F_RING_PACKED 34
#define VIRTIO_F_VERSION_1 32
#define VIRTIO_F_ACCESS_PLATFORM 33
#define VIRTIO_F_EVENT_DIX 29
#define VIRTIO_F_INDIRECT_DESC 28
#define VIRTIO_F_RING_RESET 40

#define VIRTQ_AVAIL_F_NO_INTERRUPT 1
#define VIRTQ_USED_F_NO_NOTIFY 1

#endif // __VIRTIO_H__