#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOA_DIRCR"]
    pub gpioa_dircr: GPIOA_DIRCR,
    #[doc = "0x04 - GPIOA_INER"]
    pub gpioa_iner: GPIOA_INER,
    #[doc = "0x08 - GPIOA_PUR"]
    pub gpioa_pur: GPIOA_PUR,
    #[doc = "0x0c - GPIOA_PDR"]
    pub gpioa_pdr: GPIOA_PDR,
    #[doc = "0x10 - GPIOA_ODR"]
    pub gpioa_odr: GPIOA_ODR,
    #[doc = "0x14 - GPIOA_DRVR"]
    pub gpioa_drvr: GPIOA_DRVR,
    #[doc = "0x18 - GPIOA_LOCKR"]
    pub gpioa_lockr: GPIOA_LOCKR,
    #[doc = "0x1c - GPIOA_DINR"]
    pub gpioa_dinr: GPIOA_DINR,
    #[doc = "0x20 - GPIOA_DOUTR"]
    pub gpioa_doutr: GPIOA_DOUTR,
    #[doc = "0x24 - GPIOA_SRR"]
    pub gpioa_srr: GPIOA_SRR,
    #[doc = "0x28 - GPIOA_RR"]
    pub gpioa_rr: GPIOA_RR,
}
#[doc = "GPIOA_DIRCR"]
pub struct GPIOA_DIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_DIRCR"]
pub mod gpioa_dircr;
#[doc = "GPIOA_INER"]
pub struct GPIOA_INER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_INER"]
pub mod gpioa_iner;
#[doc = "GPIOA_PUR"]
pub struct GPIOA_PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_PUR"]
pub mod gpioa_pur;
#[doc = "GPIOA_PDR"]
pub struct GPIOA_PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_PDR"]
pub mod gpioa_pdr;
#[doc = "GPIOA_ODR"]
pub struct GPIOA_ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_ODR"]
pub mod gpioa_odr;
#[doc = "GPIOA_DRVR"]
pub struct GPIOA_DRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_DRVR"]
pub mod gpioa_drvr;
#[doc = "GPIOA_LOCKR"]
pub struct GPIOA_LOCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_LOCKR"]
pub mod gpioa_lockr;
#[doc = "GPIOA_DINR"]
pub struct GPIOA_DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_DINR"]
pub mod gpioa_dinr;
#[doc = "GPIOA_DOUTR"]
pub struct GPIOA_DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_DOUTR"]
pub mod gpioa_doutr;
#[doc = "GPIOA_SRR"]
pub struct GPIOA_SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_SRR"]
pub mod gpioa_srr;
#[doc = "GPIOA_RR"]
pub struct GPIOA_RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA_RR"]
pub mod gpioa_rr;
