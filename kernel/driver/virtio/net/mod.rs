use core::ops::Add;

pub mod consts;
pub mod mmio;
pub mod pci;

use super::VirtioDevice;
use crate::driver::Device;

pub trait VirtioNetDevice {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), &'static str>;

    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
}

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

#[repr(C)]
struct VirtioNetCtrl<'a> {
    class: u8,
    cmd: u8,
    data: &'a [u8],
    ack: u8,
}

#[repr(C)]
struct VirtioNetCtrlMac {
    entries: u32,
    macs: [u8; 6],
}
