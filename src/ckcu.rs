#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CKCU_GCFGR"]
    pub ckcu_gcfgr: CKCU_GCFGR,
    #[doc = "0x04 - CKCU_GCCR"]
    pub ckcu_gccr: CKCU_GCCR,
    #[doc = "0x08 - CKCU_GCSR"]
    pub ckcu_gcsr: CKCU_GCSR,
    #[doc = "0x0c - CKCU_GCIR"]
    pub ckcu_gcir: CKCU_GCIR,
    _reserved0: [u8; 8usize],
    #[doc = "0x18 - CKCU_PLLCFGR"]
    pub ckcu_pllcfgr: CKCU_PLLCFGR,
    #[doc = "0x1c - CKCU_PLLCR"]
    pub ckcu_pllcr: CKCU_PLLCR,
    #[doc = "0x20 - CKCU_AHBCFGR"]
    pub ckcu_ahbcfgr: CKCU_AHBCFGR,
    #[doc = "0x24 - CKCU_AHBCCR"]
    pub ckcu_ahbccr: CKCU_AHBCCR,
    #[doc = "0x28 - CKCU_APBCFGR"]
    pub ckcu_apbcfgr: CKCU_APBCFGR,
    #[doc = "0x2c - CKCU_APBCCR0"]
    pub ckcu_apbccr0: CKCU_APBCCR0,
    #[doc = "0x30 - CKCU_APBCCR1"]
    pub ckcu_apbccr1: CKCU_APBCCR1,
    #[doc = "0x34 - CKCU_CKST"]
    pub ckcu_ckst: CKCU_CKST,
    #[doc = "0x38 - CKCU_APBPCSR0"]
    pub ckcu_apbpcsr0: CKCU_APBPCSR0,
    #[doc = "0x3c - CKCU_APBPCSR1"]
    pub ckcu_apbpcsr1: CKCU_APBPCSR1,
    #[doc = "0x40 - CKCU_HSICR"]
    pub ckcu_hsicr: CKCU_HSICR,
    #[doc = "0x44 - CKCU_HSIATCR"]
    pub ckcu_hsiatcr: CKCU_HSIATCR,
    _reserved1: [u8; 696usize],
    #[doc = "0x300 - CKCU_LPCR"]
    pub ckcu_lpcr: CKCU_LPCR,
    #[doc = "0x304 - CKCU_MCUDBGCR"]
    pub ckcu_mcudbgcr: CKCU_MCUDBGCR,
}
#[doc = "CKCU_GCFGR"]
pub struct CKCU_GCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_GCFGR"]
pub mod ckcu_gcfgr;
#[doc = "CKCU_GCCR"]
pub struct CKCU_GCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_GCCR"]
pub mod ckcu_gccr;
#[doc = "CKCU_GCSR"]
pub struct CKCU_GCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_GCSR"]
pub mod ckcu_gcsr;
#[doc = "CKCU_GCIR"]
pub struct CKCU_GCIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_GCIR"]
pub mod ckcu_gcir;
#[doc = "CKCU_PLLCFGR"]
pub struct CKCU_PLLCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_PLLCFGR"]
pub mod ckcu_pllcfgr;
#[doc = "CKCU_PLLCR"]
pub struct CKCU_PLLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_PLLCR"]
pub mod ckcu_pllcr;
#[doc = "CKCU_AHBCFGR"]
pub struct CKCU_AHBCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_AHBCFGR"]
pub mod ckcu_ahbcfgr;
#[doc = "CKCU_AHBCCR"]
pub struct CKCU_AHBCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_AHBCCR"]
pub mod ckcu_ahbccr;
#[doc = "CKCU_APBCFGR"]
pub struct CKCU_APBCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_APBCFGR"]
pub mod ckcu_apbcfgr;
#[doc = "CKCU_APBCCR0"]
pub struct CKCU_APBCCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_APBCCR0"]
pub mod ckcu_apbccr0;
#[doc = "CKCU_APBCCR1"]
pub struct CKCU_APBCCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_APBCCR1"]
pub mod ckcu_apbccr1;
#[doc = "CKCU_CKST"]
pub struct CKCU_CKST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_CKST"]
pub mod ckcu_ckst;
#[doc = "CKCU_APBPCSR0"]
pub struct CKCU_APBPCSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_APBPCSR0"]
pub mod ckcu_apbpcsr0;
#[doc = "CKCU_APBPCSR1"]
pub struct CKCU_APBPCSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_APBPCSR1"]
pub mod ckcu_apbpcsr1;
#[doc = "CKCU_HSICR"]
pub struct CKCU_HSICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_HSICR"]
pub mod ckcu_hsicr;
#[doc = "CKCU_HSIATCR"]
pub struct CKCU_HSIATCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_HSIATCR"]
pub mod ckcu_hsiatcr;
#[doc = "CKCU_LPCR"]
pub struct CKCU_LPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_LPCR"]
pub mod ckcu_lpcr;
#[doc = "CKCU_MCUDBGCR"]
pub struct CKCU_MCUDBGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKCU_MCUDBGCR"]
pub mod ckcu_mcudbgcr;
