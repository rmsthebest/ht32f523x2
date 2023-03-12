#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - MR0"]
    pub mr0: MR0,
    #[doc = "0x08 - MR1"]
    pub mr1: MR1,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - PR"]
    pub pr: PR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "MR0 (rw) register accessor: an alias for `Reg<MR0_SPEC>`"]
pub type MR0 = crate::Reg<mr0::MR0_SPEC>;
#[doc = "MR0"]
pub mod mr0;
#[doc = "MR1 (rw) register accessor: an alias for `Reg<MR1_SPEC>`"]
pub type MR1 = crate::Reg<mr1::MR1_SPEC>;
#[doc = "MR1"]
pub mod mr1;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "PR"]
pub mod pr;
