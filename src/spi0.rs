#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR0"]
    pub cr0: CR0,
    #[doc = "0x04 - CR1"]
    pub cr1: CR1,
    #[doc = "0x08 - IER"]
    pub ier: IER,
    #[doc = "0x0c - CPR"]
    pub cpr: CPR,
    #[doc = "0x10 - DR"]
    pub dr: DR,
    #[doc = "0x14 - SR"]
    pub sr: SR,
    #[doc = "0x18 - FCR"]
    pub fcr: FCR,
    #[doc = "0x1c - FSR"]
    pub fsr: FSR,
    #[doc = "0x20 - FTOCR"]
    pub ftocr: FTOCR,
}
#[doc = "CR0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR0"]
pub mod cr0;
#[doc = "CR1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR1"]
pub mod cr1;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "CPR"]
pub struct CPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPR"]
pub mod cpr;
#[doc = "DR"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR"]
pub mod dr;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "FCR"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FCR"]
pub mod fcr;
#[doc = "FSR"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FSR"]
pub mod fsr;
#[doc = "FTOCR"]
pub struct FTOCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTOCR"]
pub mod ftocr;
