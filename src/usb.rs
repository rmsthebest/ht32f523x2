#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSR"]
    pub csr: CSR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - ISR"]
    pub isr: ISR,
    #[doc = "0x0c - FCR"]
    pub fcr: FCR,
    #[doc = "0x10 - DEVAR"]
    pub devar: DEVAR,
    #[doc = "0x14 - EP0CSR"]
    pub ep0csr: EP0CSR,
    #[doc = "0x18 - EP0IER"]
    pub ep0ier: EP0IER,
    #[doc = "0x1c - EP0ISR"]
    pub ep0isr: EP0ISR,
    #[doc = "0x20 - EP0TCR"]
    pub ep0tcr: EP0TCR,
    #[doc = "0x24 - EP0CFGR"]
    pub ep0cfgr: EP0CFGR,
    #[doc = "0x28 - EP1CSR"]
    pub ep1csr: EP1CSR,
    #[doc = "0x2c - EP1IER"]
    pub ep1ier: EP1IER,
    #[doc = "0x30 - EP1ISR"]
    pub ep1isr: EP1ISR,
    #[doc = "0x34 - EP1TCR"]
    pub ep1tcr: EP1TCR,
    #[doc = "0x38 - EP1CFGR"]
    pub ep1cfgr: EP1CFGR,
    #[doc = "0x3c - EP2CSR"]
    pub ep2csr: EP2CSR,
    #[doc = "0x40 - EP2IER"]
    pub ep2ier: EP2IER,
    #[doc = "0x44 - EP2ISR"]
    pub ep2isr: EP2ISR,
    #[doc = "0x48 - EP2TCR"]
    pub ep2tcr: EP2TCR,
    #[doc = "0x4c - EP2CFGR"]
    pub ep2cfgr: EP2CFGR,
    #[doc = "0x50 - EP3CSR"]
    pub ep3csr: EP3CSR,
    #[doc = "0x54 - EP3IER"]
    pub ep3ier: EP3IER,
    #[doc = "0x58 - EP3ISR"]
    pub ep3isr: EP3ISR,
    #[doc = "0x5c - EP3TCR"]
    pub ep3tcr: EP3TCR,
    #[doc = "0x60 - EP3CFGR"]
    pub ep3cfgr: EP3CFGR,
    #[doc = "0x64 - EP4CSR"]
    pub ep4csr: EP4CSR,
    #[doc = "0x68 - EP4IER"]
    pub ep4ier: EP4IER,
    #[doc = "0x6c - EP4ISR"]
    pub ep4isr: EP4ISR,
    #[doc = "0x70 - EP4TCR"]
    pub ep4tcr: EP4TCR,
    #[doc = "0x74 - EP4CFGR"]
    pub ep4cfgr: EP4CFGR,
    #[doc = "0x78 - EP5CSR"]
    pub ep5csr: EP5CSR,
    #[doc = "0x7c - EP5IER"]
    pub ep5ier: EP5IER,
    #[doc = "0x80 - EP5ISR"]
    pub ep5isr: EP5ISR,
    #[doc = "0x84 - EP5TCR"]
    pub ep5tcr: EP5TCR,
    #[doc = "0x88 - EP5CFGR"]
    pub ep5cfgr: EP5CFGR,
    #[doc = "0x8c - EP6CSR"]
    pub ep6csr: EP6CSR,
    #[doc = "0x90 - EP6IER"]
    pub ep6ier: EP6IER,
    #[doc = "0x94 - EP6ISR"]
    pub ep6isr: EP6ISR,
    #[doc = "0x98 - EP6TCR"]
    pub ep6tcr: EP6TCR,
    #[doc = "0x9c - EP6CFGR"]
    pub ep6cfgr: EP6CFGR,
    #[doc = "0xa0 - EP7CSR"]
    pub ep7csr: EP7CSR,
    #[doc = "0xa4 - EP7IER"]
    pub ep7ier: EP7IER,
    #[doc = "0xa8 - EP7ISR"]
    pub ep7isr: EP7ISR,
    #[doc = "0xac - EP7TCR"]
    pub ep7tcr: EP7TCR,
    #[doc = "0xb0 - EP7CFGR"]
    pub ep7cfgr: EP7CFGR,
}
#[doc = "CSR"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSR"]
pub mod csr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "ISR"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISR"]
pub mod isr;
#[doc = "FCR"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FCR"]
pub mod fcr;
#[doc = "DEVAR"]
pub struct DEVAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DEVAR"]
pub mod devar;
#[doc = "EP0CSR"]
pub struct EP0CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP0CSR"]
pub mod ep0csr;
#[doc = "EP0IER"]
pub struct EP0IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP0IER"]
pub mod ep0ier;
#[doc = "EP0ISR"]
pub struct EP0ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP0ISR"]
pub mod ep0isr;
#[doc = "EP0TCR"]
pub struct EP0TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP0TCR"]
pub mod ep0tcr;
#[doc = "EP0CFGR"]
pub struct EP0CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP0CFGR"]
pub mod ep0cfgr;
#[doc = "EP1CSR"]
pub struct EP1CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP1CSR"]
pub mod ep1csr;
#[doc = "EP1IER"]
pub struct EP1IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP1IER"]
pub mod ep1ier;
#[doc = "EP1ISR"]
pub struct EP1ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP1ISR"]
pub mod ep1isr;
#[doc = "EP1TCR"]
pub struct EP1TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP1TCR"]
pub mod ep1tcr;
#[doc = "EP1CFGR"]
pub struct EP1CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP1CFGR"]
pub mod ep1cfgr;
#[doc = "EP2CSR"]
pub struct EP2CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP2CSR"]
pub mod ep2csr;
#[doc = "EP2IER"]
pub struct EP2IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP2IER"]
pub mod ep2ier;
#[doc = "EP2ISR"]
pub struct EP2ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP2ISR"]
pub mod ep2isr;
#[doc = "EP2TCR"]
pub struct EP2TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP2TCR"]
pub mod ep2tcr;
#[doc = "EP2CFGR"]
pub struct EP2CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP2CFGR"]
pub mod ep2cfgr;
#[doc = "EP3CSR"]
pub struct EP3CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP3CSR"]
pub mod ep3csr;
#[doc = "EP3IER"]
pub struct EP3IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP3IER"]
pub mod ep3ier;
#[doc = "EP3ISR"]
pub struct EP3ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP3ISR"]
pub mod ep3isr;
#[doc = "EP3TCR"]
pub struct EP3TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP3TCR"]
pub mod ep3tcr;
#[doc = "EP3CFGR"]
pub struct EP3CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP3CFGR"]
pub mod ep3cfgr;
#[doc = "EP4CSR"]
pub struct EP4CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP4CSR"]
pub mod ep4csr;
#[doc = "EP4IER"]
pub struct EP4IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP4IER"]
pub mod ep4ier;
#[doc = "EP4ISR"]
pub struct EP4ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP4ISR"]
pub mod ep4isr;
#[doc = "EP4TCR"]
pub struct EP4TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP4TCR"]
pub mod ep4tcr;
#[doc = "EP4CFGR"]
pub struct EP4CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP4CFGR"]
pub mod ep4cfgr;
#[doc = "EP5CSR"]
pub struct EP5CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP5CSR"]
pub mod ep5csr;
#[doc = "EP5IER"]
pub struct EP5IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP5IER"]
pub mod ep5ier;
#[doc = "EP5ISR"]
pub struct EP5ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP5ISR"]
pub mod ep5isr;
#[doc = "EP5TCR"]
pub struct EP5TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP5TCR"]
pub mod ep5tcr;
#[doc = "EP5CFGR"]
pub struct EP5CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP5CFGR"]
pub mod ep5cfgr;
#[doc = "EP6CSR"]
pub struct EP6CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP6CSR"]
pub mod ep6csr;
#[doc = "EP6IER"]
pub struct EP6IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP6IER"]
pub mod ep6ier;
#[doc = "EP6ISR"]
pub struct EP6ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP6ISR"]
pub mod ep6isr;
#[doc = "EP6TCR"]
pub struct EP6TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP6TCR"]
pub mod ep6tcr;
#[doc = "EP6CFGR"]
pub struct EP6CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP6CFGR"]
pub mod ep6cfgr;
#[doc = "EP7CSR"]
pub struct EP7CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP7CSR"]
pub mod ep7csr;
#[doc = "EP7IER"]
pub struct EP7IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP7IER"]
pub mod ep7ier;
#[doc = "EP7ISR"]
pub struct EP7ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP7ISR"]
pub mod ep7isr;
#[doc = "EP7TCR"]
pub struct EP7TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP7TCR"]
pub mod ep7tcr;
#[doc = "EP7CFGR"]
pub struct EP7CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EP7CFGR"]
pub mod ep7cfgr;
