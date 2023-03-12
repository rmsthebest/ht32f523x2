#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - SR"]
    pub sr: SR,
    #[doc = "0x08 - CCR"]
    pub ccr: CCR,
    #[doc = "0x0c - ETUR"]
    pub etur: ETUR,
    #[doc = "0x10 - GTR"]
    pub gtr: GTR,
    #[doc = "0x14 - WTR"]
    pub wtr: WTR,
    #[doc = "0x18 - IER"]
    pub ier: IER,
    #[doc = "0x1c - IPR"]
    pub ipr: IPR,
    #[doc = "0x20 - TXB"]
    pub txb: TXB,
    #[doc = "0x24 - RXB"]
    pub rxb: RXB,
    #[doc = "0x28 - PSCR"]
    pub pscr: PSCR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "CCR"]
pub mod ccr;
#[doc = "ETUR (rw) register accessor: an alias for `Reg<ETUR_SPEC>`"]
pub type ETUR = crate::Reg<etur::ETUR_SPEC>;
#[doc = "ETUR"]
pub mod etur;
#[doc = "GTR (rw) register accessor: an alias for `Reg<GTR_SPEC>`"]
pub type GTR = crate::Reg<gtr::GTR_SPEC>;
#[doc = "GTR"]
pub mod gtr;
#[doc = "WTR (rw) register accessor: an alias for `Reg<WTR_SPEC>`"]
pub type WTR = crate::Reg<wtr::WTR_SPEC>;
#[doc = "WTR"]
pub mod wtr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
#[doc = "IPR (rw) register accessor: an alias for `Reg<IPR_SPEC>`"]
pub type IPR = crate::Reg<ipr::IPR_SPEC>;
#[doc = "IPR"]
pub mod ipr;
#[doc = "TXB (rw) register accessor: an alias for `Reg<TXB_SPEC>`"]
pub type TXB = crate::Reg<txb::TXB_SPEC>;
#[doc = "TXB"]
pub mod txb;
#[doc = "RXB (rw) register accessor: an alias for `Reg<RXB_SPEC>`"]
pub type RXB = crate::Reg<rxb::RXB_SPEC>;
#[doc = "RXB"]
pub mod rxb;
#[doc = "PSCR (rw) register accessor: an alias for `Reg<PSCR_SPEC>`"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "PSCR"]
pub mod pscr;
