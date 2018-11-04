#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOD_DIRCR"]
    pub gpiod_dircr: GPIOD_DIRCR,
    #[doc = "0x04 - GPIOD_INER"]
    pub gpiod_iner: GPIOD_INER,
    #[doc = "0x08 - GPIOD_PUR"]
    pub gpiod_pur: GPIOD_PUR,
    #[doc = "0x0c - GPIOD_PDR"]
    pub gpiod_pdr: GPIOD_PDR,
    #[doc = "0x10 - GPIOD_ODR"]
    pub gpiod_odr: GPIOD_ODR,
    #[doc = "0x14 - GPIOD_DRVR"]
    pub gpiod_drvr: GPIOD_DRVR,
    #[doc = "0x18 - GPIOD_LOCKR"]
    pub gpiod_lockr: GPIOD_LOCKR,
    #[doc = "0x1c - GPIOD_DINR"]
    pub gpiod_dinr: GPIOD_DINR,
    #[doc = "0x20 - GPIOD_DOUTR"]
    pub gpiod_doutr: GPIOD_DOUTR,
    #[doc = "0x24 - GPIOD_SRR"]
    pub gpiod_srr: GPIOD_SRR,
    #[doc = "0x28 - GPIOD_RR"]
    pub gpiod_rr: GPIOD_RR,
}
#[doc = "GPIOD_DIRCR"]
pub struct GPIOD_DIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_DIRCR"]
pub mod gpiod_dircr;
#[doc = "GPIOD_INER"]
pub struct GPIOD_INER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_INER"]
pub mod gpiod_iner;
#[doc = "GPIOD_PUR"]
pub struct GPIOD_PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_PUR"]
pub mod gpiod_pur;
#[doc = "GPIOD_PDR"]
pub struct GPIOD_PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_PDR"]
pub mod gpiod_pdr;
#[doc = "GPIOD_ODR"]
pub struct GPIOD_ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_ODR"]
pub mod gpiod_odr;
#[doc = "GPIOD_DRVR"]
pub struct GPIOD_DRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_DRVR"]
pub mod gpiod_drvr;
#[doc = "GPIOD_LOCKR"]
pub struct GPIOD_LOCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_LOCKR"]
pub mod gpiod_lockr;
#[doc = "GPIOD_DINR"]
pub struct GPIOD_DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_DINR"]
pub mod gpiod_dinr;
#[doc = "GPIOD_DOUTR"]
pub struct GPIOD_DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_DOUTR"]
pub mod gpiod_doutr;
#[doc = "GPIOD_SRR"]
pub struct GPIOD_SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_SRR"]
pub mod gpiod_srr;
#[doc = "GPIOD_RR"]
pub struct GPIOD_RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOD_RR"]
pub mod gpiod_rr;
