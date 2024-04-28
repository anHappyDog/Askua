#ifndef __VIRTQ_H__
#define __VIRTQ_H__
#include <virtio/virtio.h>

#define QUEUE_SIZE 256

struct virtq {
    struct virtq_desc *desc;
    struct virtq_avail *avail;
    struct virtq_used *used;
};

struct virtq_desc {
    le64 addr;
    le32 len;
    le16 flags;
    le16 next;
}__attribute__((__aligned__(16),packed));

struct virtq_avail {
    le16 flags;
    le16 idx;
    le16 ring[QUEUE_SIZE];
    le16 used_event;
}__attribute__((__aligned__(2),packed));

struct virtq_used_elem {
    le32 id;
    le32 len;
};

struct virtq_used {
    le16 flags;
    le16 idx;
    struct virtq_used_elem ring[QUEUE_SIZE];
    le16 avail_event;
}__attribute__((__aligned__(2),packed));

#define VIRTQ_USED_F_NO_NOTIFY 1

#define VIRTQ_DESC_F_NEXT 1
#define VIRTQ_DESC_F_WRITE 2
#define VIRTQ_DESC_F_INDIRECT 4


#endif // __VIRTQ_H__