use crate::driver::virtio::pci::VirtioPCIDevice;



pub struct VirtioBlkPCIDevice {}

impl VirtioPCIDevice for VirtioBlkPCIDevice {
    fn pci_init() -> Self {
        VirtioBlkPCIDevice {}
    }
}
