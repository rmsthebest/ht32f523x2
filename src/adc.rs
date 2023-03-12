#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - LST0"]
    pub lst0: LST0,
    #[doc = "0x08 - LST1"]
    pub lst1: LST1,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - STR"]
    pub str: STR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x30 - DR0"]
    pub dr0: DR0,
    #[doc = "0x34 - DR1"]
    pub dr1: DR1,
    #[doc = "0x38 - DR2"]
    pub dr2: DR2,
    #[doc = "0x3c - DR3"]
    pub dr3: DR3,
    #[doc = "0x40 - DR4"]
    pub dr4: DR4,
    #[doc = "0x44 - DR5"]
    pub dr5: DR5,
    #[doc = "0x48 - DR6"]
    pub dr6: DR6,
    #[doc = "0x4c - DR7"]
    pub dr7: DR7,
    _reserved12: [u8; 0x20],
    #[doc = "0x70 - TCR"]
    pub tcr: TCR,
    #[doc = "0x74 - TSR"]
    pub tsr: TSR,
    #[doc = "0x78 - WCR"]
    pub wcr: WCR,
    #[doc = "0x7c - TR"]
    pub tr: TR,
    #[doc = "0x80 - IMR"]
    pub ier: IER,
    #[doc = "0x84 - IRAW"]
    pub iraw: IRAW,
    #[doc = "0x88 - ISR"]
    pub isr: ISR,
    #[doc = "0x8c - ICLR"]
    pub iclr: ICLR,
    #[doc = "0x90 - DMAR"]
    pub dmar: DMAR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "LST0 (rw) register accessor: an alias for `Reg<LST0_SPEC>`"]
pub type LST0 = crate::Reg<lst0::LST0_SPEC>;
#[doc = "LST0"]
pub mod lst0;
#[doc = "LST1 (rw) register accessor: an alias for `Reg<LST1_SPEC>`"]
pub type LST1 = crate::Reg<lst1::LST1_SPEC>;
#[doc = "LST1"]
pub mod lst1;
#[doc = "STR (rw) register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "STR"]
pub mod str;
#[doc = "DR0 (rw) register accessor: an alias for `Reg<DR0_SPEC>`"]
pub type DR0 = crate::Reg<dr0::DR0_SPEC>;
#[doc = "DR0"]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: an alias for `Reg<DR1_SPEC>`"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "DR1"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: an alias for `Reg<DR2_SPEC>`"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "DR2"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: an alias for `Reg<DR3_SPEC>`"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "DR3"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: an alias for `Reg<DR4_SPEC>`"]
pub type DR4 = crate::Reg<dr4::DR4_SPEC>;
#[doc = "DR4"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: an alias for `Reg<DR5_SPEC>`"]
pub type DR5 = crate::Reg<dr5::DR5_SPEC>;
#[doc = "DR5"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: an alias for `Reg<DR6_SPEC>`"]
pub type DR6 = crate::Reg<dr6::DR6_SPEC>;
#[doc = "DR6"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: an alias for `Reg<DR7_SPEC>`"]
pub type DR7 = crate::Reg<dr7::DR7_SPEC>;
#[doc = "DR7"]
pub mod dr7;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "TCR"]
pub mod tcr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "TSR"]
pub mod tsr;
#[doc = "WCR (rw) register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "WCR"]
pub mod wcr;
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "TR"]
pub mod tr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IMR"]
pub mod ier;
#[doc = "IRAW (rw) register accessor: an alias for `Reg<IRAW_SPEC>`"]
pub type IRAW = crate::Reg<iraw::IRAW_SPEC>;
#[doc = "IRAW"]
pub mod iraw;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ISR"]
pub mod isr;
#[doc = "ICLR (rw) register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "ICLR"]
pub mod iclr;
#[doc = "DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "DMAR"]
pub mod dmar;
