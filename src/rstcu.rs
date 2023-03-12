#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GRSR"]
    pub grsr: GRSR,
    #[doc = "0x04 - AHBPRSTR"]
    pub ahbprstr: AHBPRSTR,
    #[doc = "0x08 - APBPRSTR0"]
    pub apbprstr0: APBPRSTR0,
    #[doc = "0x0c - APBPRSTR1"]
    pub apbprstr1: APBPRSTR1,
}
#[doc = "GRSR (rw) register accessor: an alias for `Reg<GRSR_SPEC>`"]
pub type GRSR = crate::Reg<grsr::GRSR_SPEC>;
#[doc = "GRSR"]
pub mod grsr;
#[doc = "AHBPRSTR (rw) register accessor: an alias for `Reg<AHBPRSTR_SPEC>`"]
pub type AHBPRSTR = crate::Reg<ahbprstr::AHBPRSTR_SPEC>;
#[doc = "AHBPRSTR"]
pub mod ahbprstr;
#[doc = "APBPRSTR0 (rw) register accessor: an alias for `Reg<APBPRSTR0_SPEC>`"]
pub type APBPRSTR0 = crate::Reg<apbprstr0::APBPRSTR0_SPEC>;
#[doc = "APBPRSTR0"]
pub mod apbprstr0;
#[doc = "APBPRSTR1 (rw) register accessor: an alias for `Reg<APBPRSTR1_SPEC>`"]
pub type APBPRSTR1 = crate::Reg<apbprstr1::APBPRSTR1_SPEC>;
#[doc = "APBPRSTR1"]
pub mod apbprstr1;
