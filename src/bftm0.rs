#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - SR"]
    pub sr: SR,
    #[doc = "0x08 - CNTR"]
    pub cntr: CNTR,
    #[doc = "0x0c - CMPR"]
    pub cmpr: CMPR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "CNTR"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNTR"]
pub mod cntr;
#[doc = "CMPR"]
pub struct CMPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMPR"]
pub mod cmpr;
