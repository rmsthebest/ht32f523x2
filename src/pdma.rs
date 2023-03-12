#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CH0CR"]
    pub ch0cr: CH0CR,
    #[doc = "0x04 - CH0SADR"]
    pub ch0sadr: CH0SADR,
    #[doc = "0x08 - CH0DADR"]
    pub ch0dadr: CH0DADR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CH0TSR"]
    pub ch0tsr: CH0TSR,
    #[doc = "0x14 - CH0CTSR"]
    pub ch0ctsr: CH0CTSR,
    #[doc = "0x18 - CH1CR"]
    pub ch1cr: CH1CR,
    #[doc = "0x1c - CH1SADR"]
    pub ch1sadr: CH1SADR,
    #[doc = "0x20 - CH1DADR"]
    pub ch1dadr: CH1DADR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - CH1TSR"]
    pub ch1tsr: CH1TSR,
    #[doc = "0x2c - CH1CTSR"]
    pub ch1ctsr: CH1CTSR,
    #[doc = "0x30 - CH2CR"]
    pub ch2cr: CH2CR,
    #[doc = "0x34 - CH2SADR"]
    pub ch2sadr: CH2SADR,
    #[doc = "0x38 - CH2DADR"]
    pub ch2dadr: CH2DADR,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - CH2TSR"]
    pub ch2tsr: CH2TSR,
    #[doc = "0x44 - CH2CTSR"]
    pub ch2ctsr: CH2CTSR,
    #[doc = "0x48 - CH3CR"]
    pub ch3cr: CH3CR,
    #[doc = "0x4c - CH3SADR"]
    pub ch3sadr: CH3SADR,
    #[doc = "0x50 - CH3DADR"]
    pub ch3dadr: CH3DADR,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - CH3TSR"]
    pub ch3tsr: CH3TSR,
    #[doc = "0x5c - CH3CTSR"]
    pub ch3ctsr: CH3CTSR,
    #[doc = "0x60 - CH4CR"]
    pub ch4cr: CH4CR,
    #[doc = "0x64 - CH4SADR"]
    pub ch4sadr: CH4SADR,
    #[doc = "0x68 - CH4DADR"]
    pub ch4dadr: CH4DADR,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - CH4TSR"]
    pub ch4tsr: CH4TSR,
    #[doc = "0x74 - CH4CTSR"]
    pub ch4ctsr: CH4CTSR,
    #[doc = "0x78 - CH5CR"]
    pub ch5cr: CH5CR,
    #[doc = "0x7c - CH5SADR"]
    pub ch5sadr: CH5SADR,
    #[doc = "0x80 - CH5DADR"]
    pub ch5dadr: CH5DADR,
    _reserved28: [u8; 0x04],
    #[doc = "0x88 - CH5TSR"]
    pub ch5tsr: CH5TSR,
    #[doc = "0x8c - CH5CTSR"]
    pub ch5ctsr: CH5CTSR,
    _reserved30: [u8; 0x90],
    #[doc = "0x120 - ISR"]
    pub isr: ISR,
    _reserved31: [u8; 0x04],
    #[doc = "0x128 - ISCR"]
    pub iscr: ISCR,
    _reserved32: [u8; 0x04],
    #[doc = "0x130 - IER"]
    pub ier: IER,
}
#[doc = "CH0CR (rw) register accessor: an alias for `Reg<CH0CR_SPEC>`"]
pub type CH0CR = crate::Reg<ch0cr::CH0CR_SPEC>;
#[doc = "CH0CR"]
pub mod ch0cr;
#[doc = "CH0SADR (rw) register accessor: an alias for `Reg<CH0SADR_SPEC>`"]
pub type CH0SADR = crate::Reg<ch0sadr::CH0SADR_SPEC>;
#[doc = "CH0SADR"]
pub mod ch0sadr;
#[doc = "CH0DADR (rw) register accessor: an alias for `Reg<CH0DADR_SPEC>`"]
pub type CH0DADR = crate::Reg<ch0dadr::CH0DADR_SPEC>;
#[doc = "CH0DADR"]
pub mod ch0dadr;
#[doc = "CH0TSR (rw) register accessor: an alias for `Reg<CH0TSR_SPEC>`"]
pub type CH0TSR = crate::Reg<ch0tsr::CH0TSR_SPEC>;
#[doc = "CH0TSR"]
pub mod ch0tsr;
#[doc = "CH0CTSR (rw) register accessor: an alias for `Reg<CH0CTSR_SPEC>`"]
pub type CH0CTSR = crate::Reg<ch0ctsr::CH0CTSR_SPEC>;
#[doc = "CH0CTSR"]
pub mod ch0ctsr;
#[doc = "CH1CR (rw) register accessor: an alias for `Reg<CH1CR_SPEC>`"]
pub type CH1CR = crate::Reg<ch1cr::CH1CR_SPEC>;
#[doc = "CH1CR"]
pub mod ch1cr;
#[doc = "CH1SADR (rw) register accessor: an alias for `Reg<CH1SADR_SPEC>`"]
pub type CH1SADR = crate::Reg<ch1sadr::CH1SADR_SPEC>;
#[doc = "CH1SADR"]
pub mod ch1sadr;
#[doc = "CH1DADR (rw) register accessor: an alias for `Reg<CH1DADR_SPEC>`"]
pub type CH1DADR = crate::Reg<ch1dadr::CH1DADR_SPEC>;
#[doc = "CH1DADR"]
pub mod ch1dadr;
#[doc = "CH1TSR (rw) register accessor: an alias for `Reg<CH1TSR_SPEC>`"]
pub type CH1TSR = crate::Reg<ch1tsr::CH1TSR_SPEC>;
#[doc = "CH1TSR"]
pub mod ch1tsr;
#[doc = "CH1CTSR (rw) register accessor: an alias for `Reg<CH1CTSR_SPEC>`"]
pub type CH1CTSR = crate::Reg<ch1ctsr::CH1CTSR_SPEC>;
#[doc = "CH1CTSR"]
pub mod ch1ctsr;
#[doc = "CH2CR (rw) register accessor: an alias for `Reg<CH2CR_SPEC>`"]
pub type CH2CR = crate::Reg<ch2cr::CH2CR_SPEC>;
#[doc = "CH2CR"]
pub mod ch2cr;
#[doc = "CH2SADR (rw) register accessor: an alias for `Reg<CH2SADR_SPEC>`"]
pub type CH2SADR = crate::Reg<ch2sadr::CH2SADR_SPEC>;
#[doc = "CH2SADR"]
pub mod ch2sadr;
#[doc = "CH2DADR (rw) register accessor: an alias for `Reg<CH2DADR_SPEC>`"]
pub type CH2DADR = crate::Reg<ch2dadr::CH2DADR_SPEC>;
#[doc = "CH2DADR"]
pub mod ch2dadr;
#[doc = "CH2TSR (rw) register accessor: an alias for `Reg<CH2TSR_SPEC>`"]
pub type CH2TSR = crate::Reg<ch2tsr::CH2TSR_SPEC>;
#[doc = "CH2TSR"]
pub mod ch2tsr;
#[doc = "CH2CTSR (rw) register accessor: an alias for `Reg<CH2CTSR_SPEC>`"]
pub type CH2CTSR = crate::Reg<ch2ctsr::CH2CTSR_SPEC>;
#[doc = "CH2CTSR"]
pub mod ch2ctsr;
#[doc = "CH3CR (rw) register accessor: an alias for `Reg<CH3CR_SPEC>`"]
pub type CH3CR = crate::Reg<ch3cr::CH3CR_SPEC>;
#[doc = "CH3CR"]
pub mod ch3cr;
#[doc = "CH3SADR (rw) register accessor: an alias for `Reg<CH3SADR_SPEC>`"]
pub type CH3SADR = crate::Reg<ch3sadr::CH3SADR_SPEC>;
#[doc = "CH3SADR"]
pub mod ch3sadr;
#[doc = "CH3DADR (rw) register accessor: an alias for `Reg<CH3DADR_SPEC>`"]
pub type CH3DADR = crate::Reg<ch3dadr::CH3DADR_SPEC>;
#[doc = "CH3DADR"]
pub mod ch3dadr;
#[doc = "CH3TSR (rw) register accessor: an alias for `Reg<CH3TSR_SPEC>`"]
pub type CH3TSR = crate::Reg<ch3tsr::CH3TSR_SPEC>;
#[doc = "CH3TSR"]
pub mod ch3tsr;
#[doc = "CH3CTSR (rw) register accessor: an alias for `Reg<CH3CTSR_SPEC>`"]
pub type CH3CTSR = crate::Reg<ch3ctsr::CH3CTSR_SPEC>;
#[doc = "CH3CTSR"]
pub mod ch3ctsr;
#[doc = "CH4CR (rw) register accessor: an alias for `Reg<CH4CR_SPEC>`"]
pub type CH4CR = crate::Reg<ch4cr::CH4CR_SPEC>;
#[doc = "CH4CR"]
pub mod ch4cr;
#[doc = "CH4SADR (rw) register accessor: an alias for `Reg<CH4SADR_SPEC>`"]
pub type CH4SADR = crate::Reg<ch4sadr::CH4SADR_SPEC>;
#[doc = "CH4SADR"]
pub mod ch4sadr;
#[doc = "CH4DADR (rw) register accessor: an alias for `Reg<CH4DADR_SPEC>`"]
pub type CH4DADR = crate::Reg<ch4dadr::CH4DADR_SPEC>;
#[doc = "CH4DADR"]
pub mod ch4dadr;
#[doc = "CH4TSR (rw) register accessor: an alias for `Reg<CH4TSR_SPEC>`"]
pub type CH4TSR = crate::Reg<ch4tsr::CH4TSR_SPEC>;
#[doc = "CH4TSR"]
pub mod ch4tsr;
#[doc = "CH4CTSR (rw) register accessor: an alias for `Reg<CH4CTSR_SPEC>`"]
pub type CH4CTSR = crate::Reg<ch4ctsr::CH4CTSR_SPEC>;
#[doc = "CH4CTSR"]
pub mod ch4ctsr;
#[doc = "CH5CR (rw) register accessor: an alias for `Reg<CH5CR_SPEC>`"]
pub type CH5CR = crate::Reg<ch5cr::CH5CR_SPEC>;
#[doc = "CH5CR"]
pub mod ch5cr;
#[doc = "CH5SADR (rw) register accessor: an alias for `Reg<CH5SADR_SPEC>`"]
pub type CH5SADR = crate::Reg<ch5sadr::CH5SADR_SPEC>;
#[doc = "CH5SADR"]
pub mod ch5sadr;
#[doc = "CH5DADR (rw) register accessor: an alias for `Reg<CH5DADR_SPEC>`"]
pub type CH5DADR = crate::Reg<ch5dadr::CH5DADR_SPEC>;
#[doc = "CH5DADR"]
pub mod ch5dadr;
#[doc = "CH5TSR (rw) register accessor: an alias for `Reg<CH5TSR_SPEC>`"]
pub type CH5TSR = crate::Reg<ch5tsr::CH5TSR_SPEC>;
#[doc = "CH5TSR"]
pub mod ch5tsr;
#[doc = "CH5CTSR (rw) register accessor: an alias for `Reg<CH5CTSR_SPEC>`"]
pub type CH5CTSR = crate::Reg<ch5ctsr::CH5CTSR_SPEC>;
#[doc = "CH5CTSR"]
pub mod ch5ctsr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ISR"]
pub mod isr;
#[doc = "ISCR (rw) register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "ISCR"]
pub mod iscr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
