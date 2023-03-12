#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GCFGR"]
    pub gcfgr: GCFGR,
    #[doc = "0x04 - GCCR"]
    pub gccr: GCCR,
    #[doc = "0x08 - GCSR"]
    pub gcsr: GCSR,
    #[doc = "0x0c - GCIR"]
    pub gcir: GCIR,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - PLLCFGR"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x1c - PLLCR"]
    pub pllcr: PLLCR,
    #[doc = "0x20 - AHBCFGR"]
    pub ahbcfgr: AHBCFGR,
    #[doc = "0x24 - AHBCCR"]
    pub ahbccr: AHBCCR,
    #[doc = "0x28 - APBCFGR"]
    pub apbcfgr: APBCFGR,
    #[doc = "0x2c - APBCCR0"]
    pub apbccr0: APBCCR0,
    #[doc = "0x30 - APBCCR1"]
    pub apbccr1: APBCCR1,
    #[doc = "0x34 - CKST"]
    pub ckst: CKST,
    #[doc = "0x38 - APBPCSR0"]
    pub apbpcsr0: APBPCSR0,
    #[doc = "0x3c - APBPCSR1"]
    pub apbpcsr1: APBPCSR1,
    #[doc = "0x40 - HSICR"]
    pub hsicr: HSICR,
    #[doc = "0x44 - HSIATCR"]
    pub hsiatcr: HSIATCR,
    _reserved16: [u8; 0x02b8],
    #[doc = "0x300 - LPCR"]
    pub lpcr: LPCR,
    #[doc = "0x304 - MCUDBGCR"]
    pub mcudbgcr: MCUDBGCR,
}
#[doc = "GCFGR (rw) register accessor: an alias for `Reg<GCFGR_SPEC>`"]
pub type GCFGR = crate::Reg<gcfgr::GCFGR_SPEC>;
#[doc = "GCFGR"]
pub mod gcfgr;
#[doc = "GCCR (rw) register accessor: an alias for `Reg<GCCR_SPEC>`"]
pub type GCCR = crate::Reg<gccr::GCCR_SPEC>;
#[doc = "GCCR"]
pub mod gccr;
#[doc = "GCSR (rw) register accessor: an alias for `Reg<GCSR_SPEC>`"]
pub type GCSR = crate::Reg<gcsr::GCSR_SPEC>;
#[doc = "GCSR"]
pub mod gcsr;
#[doc = "GCIR (rw) register accessor: an alias for `Reg<GCIR_SPEC>`"]
pub type GCIR = crate::Reg<gcir::GCIR_SPEC>;
#[doc = "GCIR"]
pub mod gcir;
#[doc = "PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLLCFGR"]
pub mod pllcfgr;
#[doc = "PLLCR (rw) register accessor: an alias for `Reg<PLLCR_SPEC>`"]
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
#[doc = "PLLCR"]
pub mod pllcr;
#[doc = "AHBCFGR (rw) register accessor: an alias for `Reg<AHBCFGR_SPEC>`"]
pub type AHBCFGR = crate::Reg<ahbcfgr::AHBCFGR_SPEC>;
#[doc = "AHBCFGR"]
pub mod ahbcfgr;
#[doc = "AHBCCR (rw) register accessor: an alias for `Reg<AHBCCR_SPEC>`"]
pub type AHBCCR = crate::Reg<ahbccr::AHBCCR_SPEC>;
#[doc = "AHBCCR"]
pub mod ahbccr;
#[doc = "APBCFGR (rw) register accessor: an alias for `Reg<APBCFGR_SPEC>`"]
pub type APBCFGR = crate::Reg<apbcfgr::APBCFGR_SPEC>;
#[doc = "APBCFGR"]
pub mod apbcfgr;
#[doc = "APBCCR0 (rw) register accessor: an alias for `Reg<APBCCR0_SPEC>`"]
pub type APBCCR0 = crate::Reg<apbccr0::APBCCR0_SPEC>;
#[doc = "APBCCR0"]
pub mod apbccr0;
#[doc = "APBCCR1 (rw) register accessor: an alias for `Reg<APBCCR1_SPEC>`"]
pub type APBCCR1 = crate::Reg<apbccr1::APBCCR1_SPEC>;
#[doc = "APBCCR1"]
pub mod apbccr1;
#[doc = "CKST (rw) register accessor: an alias for `Reg<CKST_SPEC>`"]
pub type CKST = crate::Reg<ckst::CKST_SPEC>;
#[doc = "CKST"]
pub mod ckst;
#[doc = "APBPCSR0 (rw) register accessor: an alias for `Reg<APBPCSR0_SPEC>`"]
pub type APBPCSR0 = crate::Reg<apbpcsr0::APBPCSR0_SPEC>;
#[doc = "APBPCSR0"]
pub mod apbpcsr0;
#[doc = "APBPCSR1 (rw) register accessor: an alias for `Reg<APBPCSR1_SPEC>`"]
pub type APBPCSR1 = crate::Reg<apbpcsr1::APBPCSR1_SPEC>;
#[doc = "APBPCSR1"]
pub mod apbpcsr1;
#[doc = "HSICR (rw) register accessor: an alias for `Reg<HSICR_SPEC>`"]
pub type HSICR = crate::Reg<hsicr::HSICR_SPEC>;
#[doc = "HSICR"]
pub mod hsicr;
#[doc = "HSIATCR (rw) register accessor: an alias for `Reg<HSIATCR_SPEC>`"]
pub type HSIATCR = crate::Reg<hsiatcr::HSIATCR_SPEC>;
#[doc = "HSIATCR"]
pub mod hsiatcr;
#[doc = "LPCR (rw) register accessor: an alias for `Reg<LPCR_SPEC>`"]
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
#[doc = "LPCR"]
pub mod lpcr;
#[doc = "MCUDBGCR (rw) register accessor: an alias for `Reg<MCUDBGCR_SPEC>`"]
pub type MCUDBGCR = crate::Reg<mcudbgcr::MCUDBGCR_SPEC>;
#[doc = "MCUDBGCR"]
pub mod mcudbgcr;
