#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOC_DIRCR"]
    pub gpioc_dircr: GPIOC_DIRCR,
    #[doc = "0x04 - GPIOC_INER"]
    pub gpioc_iner: GPIOC_INER,
    #[doc = "0x08 - GPIOC_PUR"]
    pub gpioc_pur: GPIOC_PUR,
    #[doc = "0x0c - GPIOC_PDR"]
    pub gpioc_pdr: GPIOC_PDR,
    #[doc = "0x10 - GPIOC_ODR"]
    pub gpioc_odr: GPIOC_ODR,
    #[doc = "0x14 - GPIOC_DRVR"]
    pub gpioc_drvr: GPIOC_DRVR,
    #[doc = "0x18 - GPIOC_LOCKR"]
    pub gpioc_lockr: GPIOC_LOCKR,
    #[doc = "0x1c - GPIOC_DINR"]
    pub gpioc_dinr: GPIOC_DINR,
    #[doc = "0x20 - GPIOC_DOUTR"]
    pub gpioc_doutr: GPIOC_DOUTR,
    #[doc = "0x24 - GPIOC_SRR"]
    pub gpioc_srr: GPIOC_SRR,
    #[doc = "0x28 - GPIOC_RR"]
    pub gpioc_rr: GPIOC_RR,
}
#[doc = "GPIOC_DIRCR"]
pub struct GPIOC_DIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_DIRCR"]
pub mod gpioc_dircr;
#[doc = "GPIOC_INER"]
pub struct GPIOC_INER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_INER"]
pub mod gpioc_iner;
#[doc = "GPIOC_PUR"]
pub struct GPIOC_PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_PUR"]
pub mod gpioc_pur;
#[doc = "GPIOC_PDR"]
pub struct GPIOC_PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_PDR"]
pub mod gpioc_pdr;
#[doc = "GPIOC_ODR"]
pub struct GPIOC_ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_ODR"]
pub mod gpioc_odr;
#[doc = "GPIOC_DRVR"]
pub struct GPIOC_DRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_DRVR"]
pub mod gpioc_drvr;
#[doc = "GPIOC_LOCKR"]
pub struct GPIOC_LOCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_LOCKR"]
pub mod gpioc_lockr;
#[doc = "GPIOC_DINR"]
pub struct GPIOC_DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_DINR"]
pub mod gpioc_dinr;
#[doc = "GPIOC_DOUTR"]
pub struct GPIOC_DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_DOUTR"]
pub mod gpioc_doutr;
#[doc = "GPIOC_SRR"]
pub struct GPIOC_SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_SRR"]
pub mod gpioc_srr;
#[doc = "GPIOC_RR"]
pub struct GPIOC_RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOC_RR"]
pub mod gpioc_rr;
