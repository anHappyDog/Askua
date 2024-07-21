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

//ack values
const VIRTIO_NET_OK: u8 = 0;
const VIRTIO_NET_ERR: u8 = 1;

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
