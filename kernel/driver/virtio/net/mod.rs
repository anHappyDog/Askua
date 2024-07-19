use core::ops::Add;

use super::VirtioDevice;
use crate::driver::Device;

struct VirtioNetDevice {
    base: usize,
    size: usize,
}

impl VirtioDevice for VirtioNetDevice {}
impl Device for VirtioNetDevice {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add,
    {
        unsafe { ((self.base + offset) as *const T).read_volatile() }
    }

    fn write_volatile<T>(&self, offset: usize, value: T) {
        unsafe {
            ((self.base + offset) as *mut T).write_volatile(value);
        }
    }
}

const VIRTIO_NET_F_CSUM: u32 = 1 << 0;
const VIRTIO_NET_F_GUEST_CSUM: u32 = 1 << 1;
const VIRTIO_NET_F_CTRL_GUEST_OFFLOADS: u32 = 1 << 2;
const VIRTIO_NET_F_MTU: u32 = 1 << 3;
const VIRTIO_NET_F_MAC: u32 = 1 << 5;
const VIRTIO_NET_F_GUEST_TSO4: u32 = 1 << 7;
const VIRTIO_NET_F_GUEST_TSO6: u32 = 1 << 8;
const VIRTIO_NET_F_GUEST_ECN: u32 = 1 << 9;
const VIRTIO_NET_F_GUEST_UFO: u32 = 1 << 10;
const VIRTIO_NET_F_HOST_TSO4: u32 = 1 << 11;
const VIRTIO_NET_F_HOST_TSO6: u32 = 1 << 12;
const VIRTIO_NET_F_HOST_ECN: u32 = 1 << 13;
const VIRTIO_NET_F_HOST_UFO: u32 = 1 << 14;
const VIRTIO_NET_F_MRG_RXBUF: u32 = 1 << 15;
const VIRTIO_NET_F_STATUS: u32 = 1 << 16;
const VIRTIO_NET_F_CTRL_VQ: u32 = 1 << 17;
const VIRTIO_NET_F_CTRL_RX: u32 = 1 << 18;
const VIRTIO_NET_F_CTRL_VLAN: u32 = 1 << 19;
const VIRTIO_NET_F_GUEST_ANNOUNCE: u32 = 1 << 21;
const VIRTIO_NET_F_MQ: u32 = 1 << 22;
const VIRTIO_NET_F_CTRL_MAC_ADDR: u32 = 1 << 23;
// higher
const VIRTIO_NET_F_HASH_TUNNEL: u32 = 1 << 19;
const VIRTIO_NET_F_RSS: u32 = 1 << 20;
const VIRTIO_NET_F_VQ_NOTE_COAL: u32 = 1 << 21;
const VIRTIO_NET_F_GUEST_USO4: u32 = 1 << 22;
const VIRTIO_NET_F_GUEST_USO6: u32 = 1 << 23;
const VIRTIO_NET_F_HOST_USO: u32 = 1 << 24;
const VIRTIO_NET_F_HASH_REPORT: u32 = 1 << 25;
const VIRTIO_NET_F_GUEST_HDRLEN: u32 = 1 << 27;
const VIRTIO_NET_F_RSC_EXT: u32 = 1 << 29;
const VIRTIO_NET_F_STANDBY: u32 = 1 << 30;
const VIRTIO_NET_F_SPEED_DUPLEX: u32 = 1 << 31;

struct VirtioNetConfig {
    mac: [u8; 6],
    status: u16,
    max_vq_pairs: u16,
    mtu: u16,
    speed: u32,
    duplex: u8,
    rss_max_key_size: u8,
    rss_max_indirection_table_length: u16,
    support_hash_types: u32,
    supported_tunnel_types: u32,
}

const VIRTIO_NET_S_LINK_UP: u16 = 1;
const VIRTIO_NET_S_ANNOUNCE: u16 = 2;

#[repr(C)]
struct VirtioNetHdr {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
    num_buffers: u16,
    hash_value: u32,
    hash_report: u16,
    padding_reserved: u16,
}

//flags
const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8 = 1;
const VIRTIO_NET_HDR_F_DATA_VALID: u8 = 2;
const VIRTIO_NET_HDR_F_RSC_INFO: u8 = 4;
// GSO_TYPE
const VIRTIO_NET_HDR_GSO_NONE: u8 = 0;
const VIRTIO_NET_HDR_GSO_TCPV4: u8 = 1;
const VIRTIO_NET_HDR_GSO_UDP: u8 = 3;
const VIRTIO_NET_HDR_GSO_TCPV6: u8 = 4;
const VIRTIO_NET_HDR_GSO_ECN: u8 = 0x80;

#[repr(C)]
struct VirtioNetCtrl<'a> {
    class: u8,
    cmd: u8,
    data: &'a [u8],
    ack: u8,
}

//ack values
const VIRTIO_NET_OK: u8 = 0;
const VIRTIO_NET_ERR: u8 = 1;

#[repr(C)]
struct VirtioNetCtrlMac {
    entries: u32,
    macs: [u8; 6],
}
