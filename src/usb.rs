#[doc = r"Register block"]
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
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ISR"]
pub mod isr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FCR"]
pub mod fcr;
#[doc = "DEVAR (rw) register accessor: an alias for `Reg<DEVAR_SPEC>`"]
pub type DEVAR = crate::Reg<devar::DEVAR_SPEC>;
#[doc = "DEVAR"]
pub mod devar;
#[doc = "EP0CSR (rw) register accessor: an alias for `Reg<EP0CSR_SPEC>`"]
pub type EP0CSR = crate::Reg<ep0csr::EP0CSR_SPEC>;
#[doc = "EP0CSR"]
pub mod ep0csr;
#[doc = "EP0IER (rw) register accessor: an alias for `Reg<EP0IER_SPEC>`"]
pub type EP0IER = crate::Reg<ep0ier::EP0IER_SPEC>;
#[doc = "EP0IER"]
pub mod ep0ier;
#[doc = "EP0ISR (rw) register accessor: an alias for `Reg<EP0ISR_SPEC>`"]
pub type EP0ISR = crate::Reg<ep0isr::EP0ISR_SPEC>;
#[doc = "EP0ISR"]
pub mod ep0isr;
#[doc = "EP0TCR (rw) register accessor: an alias for `Reg<EP0TCR_SPEC>`"]
pub type EP0TCR = crate::Reg<ep0tcr::EP0TCR_SPEC>;
#[doc = "EP0TCR"]
pub mod ep0tcr;
#[doc = "EP0CFGR (rw) register accessor: an alias for `Reg<EP0CFGR_SPEC>`"]
pub type EP0CFGR = crate::Reg<ep0cfgr::EP0CFGR_SPEC>;
#[doc = "EP0CFGR"]
pub mod ep0cfgr;
#[doc = "EP1CSR (rw) register accessor: an alias for `Reg<EP1CSR_SPEC>`"]
pub type EP1CSR = crate::Reg<ep1csr::EP1CSR_SPEC>;
#[doc = "EP1CSR"]
pub mod ep1csr;
#[doc = "EP1IER (rw) register accessor: an alias for `Reg<EP1IER_SPEC>`"]
pub type EP1IER = crate::Reg<ep1ier::EP1IER_SPEC>;
#[doc = "EP1IER"]
pub mod ep1ier;
#[doc = "EP1ISR (rw) register accessor: an alias for `Reg<EP1ISR_SPEC>`"]
pub type EP1ISR = crate::Reg<ep1isr::EP1ISR_SPEC>;
#[doc = "EP1ISR"]
pub mod ep1isr;
#[doc = "EP1TCR (rw) register accessor: an alias for `Reg<EP1TCR_SPEC>`"]
pub type EP1TCR = crate::Reg<ep1tcr::EP1TCR_SPEC>;
#[doc = "EP1TCR"]
pub mod ep1tcr;
#[doc = "EP1CFGR (rw) register accessor: an alias for `Reg<EP1CFGR_SPEC>`"]
pub type EP1CFGR = crate::Reg<ep1cfgr::EP1CFGR_SPEC>;
#[doc = "EP1CFGR"]
pub mod ep1cfgr;
#[doc = "EP2CSR (rw) register accessor: an alias for `Reg<EP2CSR_SPEC>`"]
pub type EP2CSR = crate::Reg<ep2csr::EP2CSR_SPEC>;
#[doc = "EP2CSR"]
pub mod ep2csr;
#[doc = "EP2IER (rw) register accessor: an alias for `Reg<EP2IER_SPEC>`"]
pub type EP2IER = crate::Reg<ep2ier::EP2IER_SPEC>;
#[doc = "EP2IER"]
pub mod ep2ier;
#[doc = "EP2ISR (rw) register accessor: an alias for `Reg<EP2ISR_SPEC>`"]
pub type EP2ISR = crate::Reg<ep2isr::EP2ISR_SPEC>;
#[doc = "EP2ISR"]
pub mod ep2isr;
#[doc = "EP2TCR (rw) register accessor: an alias for `Reg<EP2TCR_SPEC>`"]
pub type EP2TCR = crate::Reg<ep2tcr::EP2TCR_SPEC>;
#[doc = "EP2TCR"]
pub mod ep2tcr;
#[doc = "EP2CFGR (rw) register accessor: an alias for `Reg<EP2CFGR_SPEC>`"]
pub type EP2CFGR = crate::Reg<ep2cfgr::EP2CFGR_SPEC>;
#[doc = "EP2CFGR"]
pub mod ep2cfgr;
#[doc = "EP3CSR (rw) register accessor: an alias for `Reg<EP3CSR_SPEC>`"]
pub type EP3CSR = crate::Reg<ep3csr::EP3CSR_SPEC>;
#[doc = "EP3CSR"]
pub mod ep3csr;
#[doc = "EP3IER (rw) register accessor: an alias for `Reg<EP3IER_SPEC>`"]
pub type EP3IER = crate::Reg<ep3ier::EP3IER_SPEC>;
#[doc = "EP3IER"]
pub mod ep3ier;
#[doc = "EP3ISR (rw) register accessor: an alias for `Reg<EP3ISR_SPEC>`"]
pub type EP3ISR = crate::Reg<ep3isr::EP3ISR_SPEC>;
#[doc = "EP3ISR"]
pub mod ep3isr;
#[doc = "EP3TCR (rw) register accessor: an alias for `Reg<EP3TCR_SPEC>`"]
pub type EP3TCR = crate::Reg<ep3tcr::EP3TCR_SPEC>;
#[doc = "EP3TCR"]
pub mod ep3tcr;
#[doc = "EP3CFGR (rw) register accessor: an alias for `Reg<EP3CFGR_SPEC>`"]
pub type EP3CFGR = crate::Reg<ep3cfgr::EP3CFGR_SPEC>;
#[doc = "EP3CFGR"]
pub mod ep3cfgr;
#[doc = "EP4CSR (rw) register accessor: an alias for `Reg<EP4CSR_SPEC>`"]
pub type EP4CSR = crate::Reg<ep4csr::EP4CSR_SPEC>;
#[doc = "EP4CSR"]
pub mod ep4csr;
#[doc = "EP4IER (rw) register accessor: an alias for `Reg<EP4IER_SPEC>`"]
pub type EP4IER = crate::Reg<ep4ier::EP4IER_SPEC>;
#[doc = "EP4IER"]
pub mod ep4ier;
#[doc = "EP4ISR (rw) register accessor: an alias for `Reg<EP4ISR_SPEC>`"]
pub type EP4ISR = crate::Reg<ep4isr::EP4ISR_SPEC>;
#[doc = "EP4ISR"]
pub mod ep4isr;
#[doc = "EP4TCR (rw) register accessor: an alias for `Reg<EP4TCR_SPEC>`"]
pub type EP4TCR = crate::Reg<ep4tcr::EP4TCR_SPEC>;
#[doc = "EP4TCR"]
pub mod ep4tcr;
#[doc = "EP4CFGR (rw) register accessor: an alias for `Reg<EP4CFGR_SPEC>`"]
pub type EP4CFGR = crate::Reg<ep4cfgr::EP4CFGR_SPEC>;
#[doc = "EP4CFGR"]
pub mod ep4cfgr;
#[doc = "EP5CSR (rw) register accessor: an alias for `Reg<EP5CSR_SPEC>`"]
pub type EP5CSR = crate::Reg<ep5csr::EP5CSR_SPEC>;
#[doc = "EP5CSR"]
pub mod ep5csr;
#[doc = "EP5IER (rw) register accessor: an alias for `Reg<EP5IER_SPEC>`"]
pub type EP5IER = crate::Reg<ep5ier::EP5IER_SPEC>;
#[doc = "EP5IER"]
pub mod ep5ier;
#[doc = "EP5ISR (rw) register accessor: an alias for `Reg<EP5ISR_SPEC>`"]
pub type EP5ISR = crate::Reg<ep5isr::EP5ISR_SPEC>;
#[doc = "EP5ISR"]
pub mod ep5isr;
#[doc = "EP5TCR (rw) register accessor: an alias for `Reg<EP5TCR_SPEC>`"]
pub type EP5TCR = crate::Reg<ep5tcr::EP5TCR_SPEC>;
#[doc = "EP5TCR"]
pub mod ep5tcr;
#[doc = "EP5CFGR (rw) register accessor: an alias for `Reg<EP5CFGR_SPEC>`"]
pub type EP5CFGR = crate::Reg<ep5cfgr::EP5CFGR_SPEC>;
#[doc = "EP5CFGR"]
pub mod ep5cfgr;
#[doc = "EP6CSR (rw) register accessor: an alias for `Reg<EP6CSR_SPEC>`"]
pub type EP6CSR = crate::Reg<ep6csr::EP6CSR_SPEC>;
#[doc = "EP6CSR"]
pub mod ep6csr;
#[doc = "EP6IER (rw) register accessor: an alias for `Reg<EP6IER_SPEC>`"]
pub type EP6IER = crate::Reg<ep6ier::EP6IER_SPEC>;
#[doc = "EP6IER"]
pub mod ep6ier;
#[doc = "EP6ISR (rw) register accessor: an alias for `Reg<EP6ISR_SPEC>`"]
pub type EP6ISR = crate::Reg<ep6isr::EP6ISR_SPEC>;
#[doc = "EP6ISR"]
pub mod ep6isr;
#[doc = "EP6TCR (rw) register accessor: an alias for `Reg<EP6TCR_SPEC>`"]
pub type EP6TCR = crate::Reg<ep6tcr::EP6TCR_SPEC>;
#[doc = "EP6TCR"]
pub mod ep6tcr;
#[doc = "EP6CFGR (rw) register accessor: an alias for `Reg<EP6CFGR_SPEC>`"]
pub type EP6CFGR = crate::Reg<ep6cfgr::EP6CFGR_SPEC>;
#[doc = "EP6CFGR"]
pub mod ep6cfgr;
#[doc = "EP7CSR (rw) register accessor: an alias for `Reg<EP7CSR_SPEC>`"]
pub type EP7CSR = crate::Reg<ep7csr::EP7CSR_SPEC>;
#[doc = "EP7CSR"]
pub mod ep7csr;
#[doc = "EP7IER (rw) register accessor: an alias for `Reg<EP7IER_SPEC>`"]
pub type EP7IER = crate::Reg<ep7ier::EP7IER_SPEC>;
#[doc = "EP7IER"]
pub mod ep7ier;
#[doc = "EP7ISR (rw) register accessor: an alias for `Reg<EP7ISR_SPEC>`"]
pub type EP7ISR = crate::Reg<ep7isr::EP7ISR_SPEC>;
#[doc = "EP7ISR"]
pub mod ep7isr;
#[doc = "EP7TCR (rw) register accessor: an alias for `Reg<EP7TCR_SPEC>`"]
pub type EP7TCR = crate::Reg<ep7tcr::EP7TCR_SPEC>;
#[doc = "EP7TCR"]
pub mod ep7tcr;
#[doc = "EP7CFGR (rw) register accessor: an alias for `Reg<EP7CFGR_SPEC>`"]
pub type EP7CFGR = crate::Reg<ep7cfgr::EP7CFGR_SPEC>;
#[doc = "EP7CFGR"]
pub mod ep7cfgr;
