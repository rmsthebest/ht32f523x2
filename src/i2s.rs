#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - CDR"]
    pub cdr: CDR,
    #[doc = "0x0c - TXDR"]
    pub txdr: TXDR,
    #[doc = "0x10 - RXDR"]
    pub rxdr: RXDR,
    #[doc = "0x14 - FCR"]
    pub fcr: FCR,
    #[doc = "0x18 - SR"]
    pub sr: SR,
    #[doc = "0x1c - RCNTR"]
    pub rcntr: RCNTR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "CDR"]
pub struct CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CDR"]
pub mod cdr;
#[doc = "TXDR"]
pub struct TXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXDR"]
pub mod txdr;
#[doc = "RXDR"]
pub struct RXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXDR"]
pub mod rxdr;
#[doc = "FCR"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FCR"]
pub mod fcr;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "RCNTR"]
pub struct RCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCNTR"]
pub mod rcntr;
