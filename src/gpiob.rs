#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOB_DIRCR"]
    pub gpiob_dircr: GPIOB_DIRCR,
    #[doc = "0x04 - GPIOB_INER"]
    pub gpiob_iner: GPIOB_INER,
    #[doc = "0x08 - GPIOB_PUR"]
    pub gpiob_pur: GPIOB_PUR,
    #[doc = "0x0c - GPIOB_PDR"]
    pub gpiob_pdr: GPIOB_PDR,
    #[doc = "0x10 - GPIOB_ODR"]
    pub gpiob_odr: GPIOB_ODR,
    #[doc = "0x14 - GPIOB_DRVR"]
    pub gpiob_drvr: GPIOB_DRVR,
    #[doc = "0x18 - GPIOB_LOCKR"]
    pub gpiob_lockr: GPIOB_LOCKR,
    #[doc = "0x1c - GPIOB_DINR"]
    pub gpiob_dinr: GPIOB_DINR,
    #[doc = "0x20 - GPIOB_DOUTR"]
    pub gpiob_doutr: GPIOB_DOUTR,
    #[doc = "0x24 - GPIOB_SRR"]
    pub gpiob_srr: GPIOB_SRR,
    #[doc = "0x28 - GPIOB_RR"]
    pub gpiob_rr: GPIOB_RR,
}
#[doc = "GPIOB_DIRCR"]
pub struct GPIOB_DIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_DIRCR"]
pub mod gpiob_dircr;
#[doc = "GPIOB_INER"]
pub struct GPIOB_INER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_INER"]
pub mod gpiob_iner;
#[doc = "GPIOB_PUR"]
pub struct GPIOB_PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_PUR"]
pub mod gpiob_pur;
#[doc = "GPIOB_PDR"]
pub struct GPIOB_PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_PDR"]
pub mod gpiob_pdr;
#[doc = "GPIOB_ODR"]
pub struct GPIOB_ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_ODR"]
pub mod gpiob_odr;
#[doc = "GPIOB_DRVR"]
pub struct GPIOB_DRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_DRVR"]
pub mod gpiob_drvr;
#[doc = "GPIOB_LOCKR"]
pub struct GPIOB_LOCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_LOCKR"]
pub mod gpiob_lockr;
#[doc = "GPIOB_DINR"]
pub struct GPIOB_DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_DINR"]
pub mod gpiob_dinr;
#[doc = "GPIOB_DOUTR"]
pub struct GPIOB_DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_DOUTR"]
pub mod gpiob_doutr;
#[doc = "GPIOB_SRR"]
pub struct GPIOB_SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_SRR"]
pub mod gpiob_srr;
#[doc = "GPIOB_RR"]
pub struct GPIOB_RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB_RR"]
pub mod gpiob_rr;
