#ifndef __VIRTIO_PCI_H__
#define __VIRTIO_PCI_H__
#include <virtio/virtio.h>
// THe structrue can be followed by extra data,depending on 'cfg_type'
struct virtio_pci_cap {
    u8 cap_vndr; // Generic PCI field: PCI_CAP_ID_VNDR
    u8 cap_next; // Next structure from the same vendor
    u8 cap_len; // Generic PCI field: capability length
    u8 cfg_type; // Identifies the structure.
    u8 bar; // Where to find it.
    u8 id; // Multiple capabilities may have the same type.
    u8 padding[2]; // Pad to full dword.
    le32 offset; // offset within bar
    le32 length; // length of the structure
};

#define VIRTIO_PCI_CAP_COMMON_CFG 1
#define VIRTIO_PCI_CAP_NOTIFY_CFG 2
#define VIRTIO_PCI_CAP_ISR_CFG 3
#define VIRTIO_PCI_CAP_DEVICE_CFG 4
#define VIRTIO_PCI_CAP_PCI_CFG 5
#define VIRTIO_PCI_CAP_SHARED_CFG 8
#define VIRTIO_PCI_CAP_VENDOR_CFG 9

//! we simply hardcode it too, later we may get it from
//! fdt parsing and device register. 
#define VIRTIO_PCI_BASE_ADDR 0x30000000
// #define VIRTIO_PCI_

#endif // __VIRTIO_PCI_H__