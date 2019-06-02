#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - LST0"]
    pub lst0: LST0,
    #[doc = "0x08 - LST1"]
    pub lst1: LST1,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - STR"]
    pub str: STR,
    _reserved1: [u8; 12usize],
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
    _reserved2: [u8; 32usize],
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
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "LST0"]
pub struct LST0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LST0"]
pub mod lst0;
#[doc = "LST1"]
pub struct LST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LST1"]
pub mod lst1;
#[doc = "STR"]
pub struct STR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STR"]
pub mod str;
#[doc = "DR0"]
pub struct DR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR0"]
pub mod dr0;
#[doc = "DR1"]
pub struct DR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR1"]
pub mod dr1;
#[doc = "DR2"]
pub struct DR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR2"]
pub mod dr2;
#[doc = "DR3"]
pub struct DR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR3"]
pub mod dr3;
#[doc = "DR4"]
pub struct DR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR4"]
pub mod dr4;
#[doc = "DR5"]
pub struct DR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR5"]
pub mod dr5;
#[doc = "DR6"]
pub struct DR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR6"]
pub mod dr6;
#[doc = "DR7"]
pub struct DR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR7"]
pub mod dr7;
#[doc = "TCR"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR"]
pub mod tcr;
#[doc = "TSR"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSR"]
pub mod tsr;
#[doc = "WCR"]
pub struct WCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCR"]
pub mod wcr;
#[doc = "TR"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TR"]
pub mod tr;
#[doc = "IMR"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IMR"]
pub mod ier;
#[doc = "IRAW"]
pub struct IRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRAW"]
pub mod iraw;
#[doc = "ISR"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISR"]
pub mod isr;
#[doc = "ICLR"]
pub struct ICLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICLR"]
pub mod iclr;
#[doc = "DMAR"]
pub struct DMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAR"]
pub mod dmar;
