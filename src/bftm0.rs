#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BFTM_CR"]
    pub bftm_cr: BFTM_CR,
    #[doc = "0x04 - BFTM_SR"]
    pub bftm_sr: BFTM_SR,
    #[doc = "0x08 - BFTM_CNTR"]
    pub bftm_cntr: BFTM_CNTR,
    #[doc = "0x0c - BFTM_CMPR"]
    pub bftm_cmpr: BFTM_CMPR,
}
#[doc = "BFTM_CR"]
pub struct BFTM_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BFTM_CR"]
pub mod bftm_cr;
#[doc = "BFTM_SR"]
pub struct BFTM_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BFTM_SR"]
pub mod bftm_sr;
#[doc = "BFTM_CNTR"]
pub struct BFTM_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BFTM_CNTR"]
pub mod bftm_cntr;
#[doc = "BFTM_CMPR"]
pub struct BFTM_CMPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BFTM_CMPR"]
pub mod bftm_cmpr;
