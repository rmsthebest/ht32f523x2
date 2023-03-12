#[doc = r"Register block"]
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
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "CR0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
#[doc = "CPR (rw) register accessor: an alias for `Reg<CPR_SPEC>`"]
pub type CPR = crate::Reg<cpr::CPR_SPEC>;
#[doc = "CPR"]
pub mod cpr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "DR"]
pub mod dr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FCR"]
pub mod fcr;
#[doc = "FSR (rw) register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "FSR"]
pub mod fsr;
#[doc = "FTOCR (rw) register accessor: an alias for `Reg<FTOCR_SPEC>`"]
pub type FTOCR = crate::Reg<ftocr::FTOCR_SPEC>;
#[doc = "FTOCR"]
pub mod ftocr;
