#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNT"]
    pub cnt: CNT,
    #[doc = "0x04 - CMP"]
    pub cmp: CMP,
    #[doc = "0x08 - CR"]
    pub cr: CR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - IWEN"]
    pub iwen: IWEN,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "CNT"]
pub mod cnt;
#[doc = "CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "CMP"]
pub mod cmp;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "IWEN (rw) register accessor: an alias for `Reg<IWEN_SPEC>`"]
pub type IWEN = crate::Reg<iwen::IWEN_SPEC>;
#[doc = "IWEN"]
pub mod iwen;
