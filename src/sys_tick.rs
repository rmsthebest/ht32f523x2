#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSR"]
    pub csr: CSR,
    #[doc = "0x04 - RVR"]
    pub rvr: RVR,
    #[doc = "0x08 - CVR"]
    pub cvr: CVR,
    #[doc = "0x0c - CALIB"]
    pub calib: CALIB,
}
#[doc = "CSR"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSR"]
pub mod csr;
#[doc = "RVR"]
pub struct RVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RVR"]
pub mod rvr;
#[doc = "CVR"]
pub struct CVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CVR"]
pub mod cvr;
#[doc = "CALIB"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CALIB"]
pub mod calib;
