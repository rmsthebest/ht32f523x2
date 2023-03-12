#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - CDR"]
    pub cdr: CDR,
    #[doc = "0x0c - TXDR"]
    pub txdr: TXDR,
    #[doc = "0x10 - RXDR"]
    pub rxdr: RXDR,
    #[doc = "0x14 - FCR"]
    pub fcr: FCR,
    #[doc = "0x18 - SR"]
    pub sr: SR,
    #[doc = "0x1c - RCNTR"]
    pub rcntr: RCNTR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
#[doc = "CDR (rw) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "CDR"]
pub mod cdr;
#[doc = "TXDR (rw) register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "TXDR"]
pub mod txdr;
#[doc = "RXDR (rw) register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "RXDR"]
pub mod rxdr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FCR"]
pub mod fcr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "RCNTR (rw) register accessor: an alias for `Reg<RCNTR_SPEC>`"]
pub type RCNTR = crate::Reg<rcntr::RCNTR_SPEC>;
#[doc = "RCNTR"]
pub mod rcntr;
