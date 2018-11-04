#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB_CSR"]
    pub usb_csr: USB_CSR,
    #[doc = "0x04 - USB_IER"]
    pub usb_ier: USB_IER,
    #[doc = "0x08 - USB_ISR"]
    pub usb_isr: USB_ISR,
    #[doc = "0x0c - USB_FCR"]
    pub usb_fcr: USB_FCR,
    #[doc = "0x10 - USB_DEVAR"]
    pub usb_devar: USB_DEVAR,
    #[doc = "0x14 - USB_EP0CSR"]
    pub usb_ep0csr: USB_EP0CSR,
    #[doc = "0x18 - USB_EP0IER"]
    pub usb_ep0ier: USB_EP0IER,
    #[doc = "0x1c - USB_EP0ISR"]
    pub usb_ep0isr: USB_EP0ISR,
    #[doc = "0x20 - USB_EP0TCR"]
    pub usb_ep0tcr: USB_EP0TCR,
    #[doc = "0x24 - USB_EP0CFGR"]
    pub usb_ep0cfgr: USB_EP0CFGR,
    #[doc = "0x28 - USB_EP1CSR"]
    pub usb_ep1csr: USB_EP1CSR,
    #[doc = "0x2c - USB_EP1IER"]
    pub usb_ep1ier: USB_EP1IER,
    #[doc = "0x30 - USB_EP1ISR"]
    pub usb_ep1isr: USB_EP1ISR,
    #[doc = "0x34 - USB_EP1TCR"]
    pub usb_ep1tcr: USB_EP1TCR,
    #[doc = "0x38 - USB_EP1CFGR"]
    pub usb_ep1cfgr: USB_EP1CFGR,
    #[doc = "0x3c - USB_EP2CSR"]
    pub usb_ep2csr: USB_EP2CSR,
    #[doc = "0x40 - USB_EP2IER"]
    pub usb_ep2ier: USB_EP2IER,
    #[doc = "0x44 - USB_EP2ISR"]
    pub usb_ep2isr: USB_EP2ISR,
    #[doc = "0x48 - USB_EP2TCR"]
    pub usb_ep2tcr: USB_EP2TCR,
    #[doc = "0x4c - USB_EP2CFGR"]
    pub usb_ep2cfgr: USB_EP2CFGR,
    #[doc = "0x50 - USB_EP3CSR"]
    pub usb_ep3csr: USB_EP3CSR,
    #[doc = "0x54 - USB_EP3IER"]
    pub usb_ep3ier: USB_EP3IER,
    #[doc = "0x58 - USB_EP3ISR"]
    pub usb_ep3isr: USB_EP3ISR,
    #[doc = "0x5c - USB_EP3TCR"]
    pub usb_ep3tcr: USB_EP3TCR,
    #[doc = "0x60 - USB_EP3CFGR"]
    pub usb_ep3cfgr: USB_EP3CFGR,
    #[doc = "0x64 - USB_EP4CSR"]
    pub usb_ep4csr: USB_EP4CSR,
    #[doc = "0x68 - USB_EP4IER"]
    pub usb_ep4ier: USB_EP4IER,
    #[doc = "0x6c - USB_EP4ISR"]
    pub usb_ep4isr: USB_EP4ISR,
    #[doc = "0x70 - USB_EP4TCR"]
    pub usb_ep4tcr: USB_EP4TCR,
    #[doc = "0x74 - USB_EP4CFGR"]
    pub usb_ep4cfgr: USB_EP4CFGR,
    #[doc = "0x78 - USB_EP5CSR"]
    pub usb_ep5csr: USB_EP5CSR,
    #[doc = "0x7c - USB_EP5IER"]
    pub usb_ep5ier: USB_EP5IER,
    #[doc = "0x80 - USB_EP5ISR"]
    pub usb_ep5isr: USB_EP5ISR,
    #[doc = "0x84 - USB_EP5TCR"]
    pub usb_ep5tcr: USB_EP5TCR,
    #[doc = "0x88 - USB_EP5CFGR"]
    pub usb_ep5cfgr: USB_EP5CFGR,
    #[doc = "0x8c - USB_EP6CSR"]
    pub usb_ep6csr: USB_EP6CSR,
    #[doc = "0x90 - USB_EP6IER"]
    pub usb_ep6ier: USB_EP6IER,
    #[doc = "0x94 - USB_EP6ISR"]
    pub usb_ep6isr: USB_EP6ISR,
    #[doc = "0x98 - USB_EP6TCR"]
    pub usb_ep6tcr: USB_EP6TCR,
    #[doc = "0x9c - USB_EP6CFGR"]
    pub usb_ep6cfgr: USB_EP6CFGR,
    #[doc = "0xa0 - USB_EP7CSR"]
    pub usb_ep7csr: USB_EP7CSR,
    #[doc = "0xa4 - USB_EP7IER"]
    pub usb_ep7ier: USB_EP7IER,
    #[doc = "0xa8 - USB_EP7ISR"]
    pub usb_ep7isr: USB_EP7ISR,
    #[doc = "0xac - USB_EP7TCR"]
    pub usb_ep7tcr: USB_EP7TCR,
    #[doc = "0xb0 - USB_EP7CFGR"]
    pub usb_ep7cfgr: USB_EP7CFGR,
}
#[doc = "USB_CSR"]
pub struct USB_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_CSR"]
pub mod usb_csr;
#[doc = "USB_IER"]
pub struct USB_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_IER"]
pub mod usb_ier;
#[doc = "USB_ISR"]
pub struct USB_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_ISR"]
pub mod usb_isr;
#[doc = "USB_FCR"]
pub struct USB_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_FCR"]
pub mod usb_fcr;
#[doc = "USB_DEVAR"]
pub struct USB_DEVAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_DEVAR"]
pub mod usb_devar;
#[doc = "USB_EP0CSR"]
pub struct USB_EP0CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP0CSR"]
pub mod usb_ep0csr;
#[doc = "USB_EP0IER"]
pub struct USB_EP0IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP0IER"]
pub mod usb_ep0ier;
#[doc = "USB_EP0ISR"]
pub struct USB_EP0ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP0ISR"]
pub mod usb_ep0isr;
#[doc = "USB_EP0TCR"]
pub struct USB_EP0TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP0TCR"]
pub mod usb_ep0tcr;
#[doc = "USB_EP0CFGR"]
pub struct USB_EP0CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP0CFGR"]
pub mod usb_ep0cfgr;
#[doc = "USB_EP1CSR"]
pub struct USB_EP1CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP1CSR"]
pub mod usb_ep1csr;
#[doc = "USB_EP1IER"]
pub struct USB_EP1IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP1IER"]
pub mod usb_ep1ier;
#[doc = "USB_EP1ISR"]
pub struct USB_EP1ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP1ISR"]
pub mod usb_ep1isr;
#[doc = "USB_EP1TCR"]
pub struct USB_EP1TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP1TCR"]
pub mod usb_ep1tcr;
#[doc = "USB_EP1CFGR"]
pub struct USB_EP1CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP1CFGR"]
pub mod usb_ep1cfgr;
#[doc = "USB_EP2CSR"]
pub struct USB_EP2CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP2CSR"]
pub mod usb_ep2csr;
#[doc = "USB_EP2IER"]
pub struct USB_EP2IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP2IER"]
pub mod usb_ep2ier;
#[doc = "USB_EP2ISR"]
pub struct USB_EP2ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP2ISR"]
pub mod usb_ep2isr;
#[doc = "USB_EP2TCR"]
pub struct USB_EP2TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP2TCR"]
pub mod usb_ep2tcr;
#[doc = "USB_EP2CFGR"]
pub struct USB_EP2CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP2CFGR"]
pub mod usb_ep2cfgr;
#[doc = "USB_EP3CSR"]
pub struct USB_EP3CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP3CSR"]
pub mod usb_ep3csr;
#[doc = "USB_EP3IER"]
pub struct USB_EP3IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP3IER"]
pub mod usb_ep3ier;
#[doc = "USB_EP3ISR"]
pub struct USB_EP3ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP3ISR"]
pub mod usb_ep3isr;
#[doc = "USB_EP3TCR"]
pub struct USB_EP3TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP3TCR"]
pub mod usb_ep3tcr;
#[doc = "USB_EP3CFGR"]
pub struct USB_EP3CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP3CFGR"]
pub mod usb_ep3cfgr;
#[doc = "USB_EP4CSR"]
pub struct USB_EP4CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP4CSR"]
pub mod usb_ep4csr;
#[doc = "USB_EP4IER"]
pub struct USB_EP4IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP4IER"]
pub mod usb_ep4ier;
#[doc = "USB_EP4ISR"]
pub struct USB_EP4ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP4ISR"]
pub mod usb_ep4isr;
#[doc = "USB_EP4TCR"]
pub struct USB_EP4TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP4TCR"]
pub mod usb_ep4tcr;
#[doc = "USB_EP4CFGR"]
pub struct USB_EP4CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP4CFGR"]
pub mod usb_ep4cfgr;
#[doc = "USB_EP5CSR"]
pub struct USB_EP5CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP5CSR"]
pub mod usb_ep5csr;
#[doc = "USB_EP5IER"]
pub struct USB_EP5IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP5IER"]
pub mod usb_ep5ier;
#[doc = "USB_EP5ISR"]
pub struct USB_EP5ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP5ISR"]
pub mod usb_ep5isr;
#[doc = "USB_EP5TCR"]
pub struct USB_EP5TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP5TCR"]
pub mod usb_ep5tcr;
#[doc = "USB_EP5CFGR"]
pub struct USB_EP5CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP5CFGR"]
pub mod usb_ep5cfgr;
#[doc = "USB_EP6CSR"]
pub struct USB_EP6CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP6CSR"]
pub mod usb_ep6csr;
#[doc = "USB_EP6IER"]
pub struct USB_EP6IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP6IER"]
pub mod usb_ep6ier;
#[doc = "USB_EP6ISR"]
pub struct USB_EP6ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP6ISR"]
pub mod usb_ep6isr;
#[doc = "USB_EP6TCR"]
pub struct USB_EP6TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP6TCR"]
pub mod usb_ep6tcr;
#[doc = "USB_EP6CFGR"]
pub struct USB_EP6CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP6CFGR"]
pub mod usb_ep6cfgr;
#[doc = "USB_EP7CSR"]
pub struct USB_EP7CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP7CSR"]
pub mod usb_ep7csr;
#[doc = "USB_EP7IER"]
pub struct USB_EP7IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP7IER"]
pub mod usb_ep7ier;
#[doc = "USB_EP7ISR"]
pub struct USB_EP7ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP7ISR"]
pub mod usb_ep7isr;
#[doc = "USB_EP7TCR"]
pub struct USB_EP7TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP7TCR"]
pub mod usb_ep7tcr;
#[doc = "USB_EP7CFGR"]
pub struct USB_EP7CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB_EP7CFGR"]
pub mod usb_ep7cfgr;
