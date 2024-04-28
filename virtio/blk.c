#include <lock/mutex.h>
#include <lock/spinlock.h>
#include <mm/mm.h>
#include <printk.h>
#include <string.h>
#include <virtio/virtio_blk.h>
#include <virtio/virtq.h>

static struct virtq vq;
static spinlock_t desc_lock = SPIN_INIT;
static u8 desc_bitmap[ROUNDDOWN(QUEUE_SIZE, 8) >> 3] = {0};

//! acutally the vq ,the desc alloc, the fs all need locks,but the type if different.
//! we now only set lock for the desc alloc.
//! Moreover, the lock need to handle the interrupt but we currently not.
uint32_t virtio_desc_alloc()
{

	int alloc = -1;
	spin_lock(&desc_lock);
	for (int i = 0; i < ROUNDDOWN(QUEUE_SIZE, 8) >> 3; ++i) {
		if (desc_bitmap[i] != 0xff) {
			for (int j = 0; j < 8; ++j) {
				if (!(desc_bitmap[i] & (1 << j))) {
					desc_bitmap[i] |= (1 << j);
					alloc = i * 8 + j;
					break;
				}
			}
			break;
		}
	}
	spin_unlock(&desc_lock);
	return alloc;
}

void virtio_desc_free(const le16 *allocs, size_t length)
{
	spin_lock(&desc_lock);
	for (int i = 0; i < length; ++i) {
		desc_bitmap[allocs[i] >> 3] &= ~(1 << (allocs[i] & 0x7));
	}
	spin_unlock(&desc_lock);
}
void virtio_blk_init(size_t base)
{
	u32 features, status = 0;

	if (READ_MMIO_REG(base, MMIO_MAGIC_OFFST, u32) != VIRTIO_MMIO_MAGIC) {
		panic("virtio_blk: wrong magic number\n");
	}
	if (READ_MMIO_REG(base, MMIO_VERSION_OFFST, u32) != VIRTIO_MMIO_VERSION) {
		panic("virtio_blk: wrong version\n");
	}
	if (READ_MMIO_REG(base, MMIO_DEVICE_ID_OFFST, u32) == 0) {
		panic("virtio_blk: wrong device id\n");
	}
	WRITE_MMIO_REG(base, MMIO_STATUS_OFFST, u32, 0);
	status |= DRIVER;
	WRITE_MMIO_REG(base, MMIO_STATUS_OFFST, u32, status);
	WRITE_MMIO_REG(base, MMIO_DEVICE_FEATURES_SEL_OFFST, u32, 0);
	features = READ_MMIO_REG(base, MMIO_DEVICE_FEATURES_OFFST, u32);
	// printk("virtio_blk: device features lower 32: %x\n", features);
	// features &= ~VIRTIO_F_NOTIFICATION_DATA;
	features &= ~(1 << VIRTIO_BLK_F_RO);
	// features &= ~(1 << VIRTIO_BLK_F_MQ);
	features &= ~(1 << VIRTIO_F_INDIRECT_DESC);
	features &= ~(1 << VIRTIO_F_EVENT_DIX);

	// pick features you want
	WRITE_MMIO_REG(base, MMIO_DRIVER_FEATURES_SEL_OFFST, u32, 0);
	WRITE_MMIO_REG(base, MMIO_DRIVER_FEATURES_OFFST, u32, features);
	// printk("virtio_blk: driver features lower 32: %x\n", features);
	WRITE_MMIO_REG(base, MMIO_DEVICE_FEATURES_SEL_OFFST, u32, 1);
	features = READ_MMIO_REG(base, MMIO_DEVICE_FEATURES_OFFST, u32);
	// printk("virtio_blk: device features higher 32: %x\n", features);
	// pick features you want
	// features = ~(1 << (VIRTIO_F_NOTIFICATION_DATA - 32));
	// features &= ~(1 << (VIRTIO_F_RING_PACKED - 32));
	WRITE_MMIO_REG(base, MMIO_DRIVER_FEATURES_SEL_OFFST, u32, 1);
	WRITE_MMIO_REG(base, MMIO_DRIVER_FEATURES_OFFST, u32, features);
	// printk("virtio_blk: driver features higher 32: %08xx\n", features);
	status |= FEATURES_OK;
	WRITE_MMIO_REG(base, MMIO_STATUS_OFFST, u32, status);
	if ((READ_MMIO_REG(base, MMIO_STATUS_OFFST, u32) & FEATURES_OK) == 0) {
		panic("virtio_blk: features negotiation failed\n");
	}
	// do blk mmio specific init
	WRITE_MMIO_REG(base, MMIO_QUEUE_SEL_OFFST, u32, 0);
	if (READ_MMIO_REG(base, MMIO_QUEUE_READY_OFFST, u32) != 0) {
		panic("virtio_blk: queue 0 is not ready\n");
	}
	// printk("virtio_blk: QUEUE_NUM_MAX is %08x\n",
	//        READ_MMIO_REG(base, MMIO_QUEUE_NUM_MAX_OFFST, u32));
	if (READ_MMIO_REG(base, MMIO_QUEUE_NUM_MAX_OFFST, u32) < QUEUE_SIZE) {
		panic("virtio_blk: queue 0 is not available\n");
	}
	WRITE_MMIO_REG(base, MMIO_QUEUE_NUM_OFFST, u32, QUEUE_SIZE);
	// ALLOC PAGES FOR  the queue
	// vq.avail = (struct virtq_avail *)(alloc_pages(0) | VIRTUAL_KERNEL_BASE);
	memset(vq.avail, 0, PAGE_SIZE);
	// vq.desc = (struct virtq_desc *)(alloc_pages(0) | VIRTUAL_KERNEL_BASE);
	memset(vq.desc, 0, PAGE_SIZE);
	// vq.used = (struct virtq_used *)(alloc_pages(0) | VIRTUAL_KERNEL_BASE);
	memset(vq.used, 0, PAGE_SIZE);
	// vq.avail->flags = VIRTQ_AVAIL_F_NO_INTERRUPT;
	WRITE_MMIO_REG(base, MMIO_QUEUE_DESC_LOW_OFFST, u32, ((size_t)vq.desc) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_DESC_HIGH_OFFST, u32,
		       (((size_t)vq.desc) >> 32) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_DRIVER_LOW_OFFST, u32, ((size_t)vq.avail) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_DRIVER_HIGH_OFFST, u32,
		       (((size_t)vq.avail) >> 32) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_DEVICE_LOW_OFFST, u32, ((size_t)vq.used) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_DEVICE_HIGH_OFFST, u32,
		       (((size_t)vq.used) >> 32) & 0xffffffff);
	WRITE_MMIO_REG(base, MMIO_QUEUE_READY_OFFST, u32, 0x1);
	status |= DRIVER_OK;
	WRITE_MMIO_REG(base, MMIO_STATUS_OFFST, u32, status);
	printk("virtio_blk: init finished.\n");
}

#define MAX_USED_DESC_ONCE_CNT 12

void virtio_blk_rw_sectors(size_t base, size_t sector, size_t count, void *buf, uint8_t w)
{
	u32 desc_idx;
	le16 allocs[MAX_USED_DESC_ONCE_CNT];
	le16 usd_desc = 0, idx = 0;
	struct virtq_desc *desc;
	struct virtio_blk_req rq = {
	    .type = w ? VIRTIO_BLK_T_OUT : VIRTIO_BLK_T_IN,
	    .sector = sector,
	    .reserved = 0,
	    .data = (void *)buf,
	    .status = 0x3,
	};
	allocs[usd_desc++] = desc_idx = virtio_desc_alloc();
	desc = &vq.desc[desc_idx];
	desc->addr = (size_t)&rq;
	desc->len = 16;
	desc->flags = VIRTQ_DESC_F_NEXT;
	allocs[usd_desc++] = desc_idx = virtio_desc_alloc();
	desc->next = desc_idx;
	desc = &vq.desc[desc->next];
	for (int i = 0; i < count; ++i) {
		desc->addr = (size_t)(buf + i * 512);
		desc->len = 1 * 512; // count
		desc->flags = (w ? 0 : VIRTQ_DESC_F_WRITE) | VIRTQ_DESC_F_NEXT;
		allocs[usd_desc++] = desc_idx = virtio_desc_alloc();
		desc->next = desc_idx;
		desc = &vq.desc[desc->next];
	}
	desc->addr = (size_t)&rq.status;
	desc->len = 1;
	desc->flags = VIRTQ_DESC_F_WRITE;
	desc->next = 0;
	idx = vq.avail->idx;
	// RISCV_FENCE(rw,rw);
	vq.avail->ring[idx] = 0;
	// RISCV_FENCE(rw,rw);
	vq.avail->idx = (idx + 1) % QUEUE_SIZE;

	// RISCV_FENCE(rw,rw);
	WRITE_MMIO_REG(base, MMIO_QUEUE_NOTIFY_OFFST, u32, 0);
	asm volatile("wfi");
	//& here we can also use the pull to replace the interrupt
	// while (1) {
	// 	WRITE_MMIO_REG(base,MMIO_INTERRUPT_ACK_OFFST,u32,READ_MMIO_REG(base,MMIO_INTERRUPT_STATUS_OFFST,u32)
	// - & 0x3); 	if (rq.status != 0x3) { 		break;
	// 	}
	// }
	virtio_desc_free(allocs, usd_desc);
}
