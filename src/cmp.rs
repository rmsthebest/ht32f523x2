#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR0"]
    pub cr0: CR0,
    #[doc = "0x04 - VALR0"]
    pub valr0: VALR0,
    #[doc = "0x08 - IER0"]
    pub ier0: IER0,
    #[doc = "0x0c - TFR0"]
    pub tfr0: TFR0,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - CR1"]
    pub cr1: CR1,
    #[doc = "0x104 - VALR1"]
    pub valr1: VALR1,
    #[doc = "0x108 - IER1"]
    pub ier1: IER1,
    #[doc = "0x10c - TFR1"]
    pub tfr1: TFR1,
}
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "CR0"]
pub mod cr0;
#[doc = "VALR0 (rw) register accessor: an alias for `Reg<VALR0_SPEC>`"]
pub type VALR0 = crate::Reg<valr0::VALR0_SPEC>;
#[doc = "VALR0"]
pub mod valr0;
#[doc = "IER0 (rw) register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "IER0"]
pub mod ier0;
#[doc = "TFR0 (rw) register accessor: an alias for `Reg<TFR0_SPEC>`"]
pub type TFR0 = crate::Reg<tfr0::TFR0_SPEC>;
#[doc = "TFR0"]
pub mod tfr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "VALR1 (rw) register accessor: an alias for `Reg<VALR1_SPEC>`"]
pub type VALR1 = crate::Reg<valr1::VALR1_SPEC>;
#[doc = "VALR1"]
pub mod valr1;
#[doc = "IER1 (rw) register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "IER1"]
pub mod ier1;
#[doc = "TFR1 (rw) register accessor: an alias for `Reg<TFR1_SPEC>`"]
pub type TFR1 = crate::Reg<tfr1::TFR1_SPEC>;
#[doc = "TFR1"]
pub mod tfr1;
