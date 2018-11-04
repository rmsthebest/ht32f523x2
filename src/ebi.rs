#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBI_CR"]
    pub ebi_cr: EBI_CR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - EBI_SR"]
    pub ebi_sr: EBI_SR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - EBI_ATR"]
    pub ebi_atr: EBI_ATR,
    #[doc = "0x14 - EBI_RTR"]
    pub ebi_rtr: EBI_RTR,
    #[doc = "0x18 - EBI_WTR"]
    pub ebi_wtr: EBI_WTR,
    #[doc = "0x1c - EBI_PR"]
    pub ebi_pr: EBI_PR,
}
#[doc = "EBI_CR"]
pub struct EBI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_CR"]
pub mod ebi_cr;
#[doc = "EBI_SR"]
pub struct EBI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_SR"]
pub mod ebi_sr;
#[doc = "EBI_ATR"]
pub struct EBI_ATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_ATR"]
pub mod ebi_atr;
#[doc = "EBI_RTR"]
pub struct EBI_RTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_RTR"]
pub mod ebi_rtr;
#[doc = "EBI_WTR"]
pub struct EBI_WTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_WTR"]
pub mod ebi_wtr;
#[doc = "EBI_PR"]
pub struct EBI_PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBI_PR"]
pub mod ebi_pr;
