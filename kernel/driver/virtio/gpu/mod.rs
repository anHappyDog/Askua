use core::ops::Add;

use super::VirtioDevice;
use crate::driver::Device;

const VIRTIO_GPU_F_VIRGL: u32 = 0;
const VIRTIO_GPU_F_EDIE: u32 = 1;
const VIRTIO_GPU_F_RESOURCE_UUID: u32 = 2;
const VIRTIO_GPU_F_RESOURCE_BLOB: u32 = 3;
const VIRTIO_GPU_F_CONTEXT_INIT: u32 = 4;

struct VirtioGpuDevice {
    base: usize,
    size: usize,
}
impl VirtioDevice for VirtioGpuDevice {}

impl Device for VirtioGpuDevice {
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

struct VirtioGpuConfig {
    events_read: u32,
    eventes_clear: u32,
    num_scanouts: u32,
    num_capsets: u32,
}

enum VirtioGpuShmId {
    VirtioGpuShmIdUndefined = 0,
    VirtioGpuShmIdHostVisible = 1,
}

enum VirtioGpuCtrlType {
    VirtioGpuCmdGetDisplayInfo = 0x0100,
    VirtioGpuCmdResourceCreate2d,
    VirtioGpuCmdResourceUnref,
    VirtioGpuCmdSetScanout,
    VirtioGpuCmdResourceFlush,
    VirtioGpuCmdTransferToHost2d,
    VirtioGpuCmdResourceAttachBacking,
    VirtioGpuCmdResourceDetachBacking,
    VirtioGpuCmdGetCapsetInfo,
    VirtioGpuCmdGetCapset,
    VirtioGpuCmdGetEdid,
    VirtioGpuCmdResourceAssignUuid,
    VirtioGpuCmdResourceCreateBlob,
    VirtioGpuCmdSetScanoutBlob,
    VirtioGpuCmdCtxCreate = 0x0200,
    VirtioGpuCmdCtxDestroy,
    VirtioGpuCmdCtxAttachResource,
    VirtioGpuCmdCtxDetachResource,
    VirtioGpuCmdResourceCreate3d,
    VirtioGpuCmdTransferToHost3d,
    VirtioGpuCmdTransferFromHost3d,
    VirtioGpuCmdSubmit3d,
    VirtioGpuCmdResourceMapBlob,
    VirtioGpuCmdResourceUnmapBlob,
    /* cursor commands */
    VirtioGpuCmdUpdateCursor = 0x0300,
    VirtioGpuCmdMoveCursor,
    // success responses
    VirtioGpuRespOkNodata = 0x1100,
    VirtioGpuRespOkDisplayInfo,
    VirtioGpuRespOkCapsetInfo,
    VirtioGpuRespOkCapset,
    VirtioGpuRespOkEdid,
    VirtioGpuRespOkResourceUuid,
    VirtioGpuRespOkMapInfo,
    /* error responses */
    VirtioGpuRespErrUnspec = 0x1200,
    VirtioGpuRespErrOutOfMemory,
    VirtioGpuRespErrInvalidScanoutId,
    VirtioGpuRespErrInvalidResourceId,
    VirtioGpuRespErrInvalidContextId,
    VirtioGpuRespErrInvalidParameter,
}

const VIRTIO_GPU_FLAG_FENCE: u32 = 1 << 0;
const VIRTIO_GPU_FLAG_INFO_RING_IDX: u32 = 1 << 1;

pub struct VirtioGpuCtrlHdr {
    type_: u32,
    flags: u32,
    fence_id: u64,
    ctx_id: u32,
    ring_idx: u8,
    pad: [u8; 3],
}

const VIRTIO_GPU_MAX_SCANOUTS: u32 = 16;

struct VirtioGpuRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

struct VirtioGpuDisplayOne {
    r: VirtioGpuRect,
    enabled: u32,
    flags: u32,
}

struct VirtioGpuRespDisplayInfo {
    hdr: VirtioGpuCtrlHdr,
    pmodes: [VirtioGpuDisplayOne; VIRTIO_GPU_MAX_SCANOUTS as usize],
}

struct VirtioGpuGetEdit {
    hdr: VirtioGpuCtrlHdr,
    scanout: u32,
    pad: u32,
}
struct VirtioGpuRespEdit {
    hdr: VirtioGpuCtrlHdr,
    size: u32,
    padding: u32,
    edid: [u8; 1024],
}

struct VirtioGpuResourceUnref {
    hdr: VirtioGpuCtrlHdr,
    resource_id: u32,
    padding: u32,
}

struct VirtioGpuSetScanout {
    hdr: VirtioGpuCtrlHdr,
    r: VirtioGpuRect,
    scanout_id: u32,
    resource_id: u32,
}

struct VirtioGpuResourceFlush {
    hdr: VirtioGpuCtrlHdr,
    r: VirtioGpuRect,
    resource_id: u32,
    padding: u32,
}

struct VirtioGpuResourceAttachBacking {
    hdr: VirtioGpuCtrlHdr,
    resource_id: u32,
    nr_entries: u32,
}

struct VirtioGpuMemEntry {
    addr: u64,
    length: u32,
    padding: u32,
}

struct VirtioGpuResourceDetachBacking {
    hdr: VirtioGpuCtrlHdr,
    resource_id: u32,
    padding: u32,
}


const DEVICE_ID : u32 = 18;

