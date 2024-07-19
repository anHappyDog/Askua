use crate::driver::virtio::VirtioPCIDevice;

pub struct VirtioBlkPCIDevice {}

impl VirtioPCIDevice for VirtioBlkPCIDevice {
    fn pci_init() -> Self {
        VirtioBlkPCIDevice {}
    }
}
