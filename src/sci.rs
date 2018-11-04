#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCI_CR"]
    pub sci_cr: SCI_CR,
    #[doc = "0x04 - SCI_SR"]
    pub sci_sr: SCI_SR,
    #[doc = "0x08 - SCI_CCR"]
    pub sci_ccr: SCI_CCR,
    #[doc = "0x0c - SCI_ETUR"]
    pub sci_etur: SCI_ETUR,
    #[doc = "0x10 - SCI_GTR"]
    pub sci_gtr: SCI_GTR,
    #[doc = "0x14 - SCI_WTR"]
    pub sci_wtr: SCI_WTR,
    #[doc = "0x18 - SCI_IER"]
    pub sci_ier: SCI_IER,
    #[doc = "0x1c - SCI_IPR"]
    pub sci_ipr: SCI_IPR,
    #[doc = "0x20 - SCI_TXB"]
    pub sci_txb: SCI_TXB,
    #[doc = "0x24 - SCI_RXB"]
    pub sci_rxb: SCI_RXB,
    #[doc = "0x28 - SCI_PSCR"]
    pub sci_pscr: SCI_PSCR,
}
#[doc = "SCI_CR"]
pub struct SCI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_CR"]
pub mod sci_cr;
#[doc = "SCI_SR"]
pub struct SCI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_SR"]
pub mod sci_sr;
#[doc = "SCI_CCR"]
pub struct SCI_CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_CCR"]
pub mod sci_ccr;
#[doc = "SCI_ETUR"]
pub struct SCI_ETUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_ETUR"]
pub mod sci_etur;
#[doc = "SCI_GTR"]
pub struct SCI_GTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_GTR"]
pub mod sci_gtr;
#[doc = "SCI_WTR"]
pub struct SCI_WTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_WTR"]
pub mod sci_wtr;
#[doc = "SCI_IER"]
pub struct SCI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_IER"]
pub mod sci_ier;
#[doc = "SCI_IPR"]
pub struct SCI_IPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_IPR"]
pub mod sci_ipr;
#[doc = "SCI_TXB"]
pub struct SCI_TXB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_TXB"]
pub mod sci_txb;
#[doc = "SCI_RXB"]
pub struct SCI_RXB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_RXB"]
pub mod sci_rxb;
#[doc = "SCI_PSCR"]
pub struct SCI_PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI_PSCR"]
pub mod sci_pscr;
