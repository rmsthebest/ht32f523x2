#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - SR"]
    pub sr: SR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - ATR"]
    pub atr: ATR,
    #[doc = "0x14 - RTR"]
    pub rtr: RTR,
    #[doc = "0x18 - WTR"]
    pub wtr: WTR,
    #[doc = "0x1c - PR"]
    pub pr: PR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "ATR (rw) register accessor: an alias for `Reg<ATR_SPEC>`"]
pub type ATR = crate::Reg<atr::ATR_SPEC>;
#[doc = "ATR"]
pub mod atr;
#[doc = "RTR (rw) register accessor: an alias for `Reg<RTR_SPEC>`"]
pub type RTR = crate::Reg<rtr::RTR_SPEC>;
#[doc = "RTR"]
pub mod rtr;
#[doc = "WTR (rw) register accessor: an alias for `Reg<WTR_SPEC>`"]
pub type WTR = crate::Reg<wtr::WTR_SPEC>;
#[doc = "WTR"]
pub mod wtr;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "PR"]
pub mod pr;
