#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA_CH0CR"]
    pub pdma_ch0cr: PDMA_CH0CR,
    #[doc = "0x04 - PDMA_CH0SADR"]
    pub pdma_ch0sadr: PDMA_CH0SADR,
    #[doc = "0x08 - PDMA_CH0DADR"]
    pub pdma_ch0dadr: PDMA_CH0DADR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - PDMA_CH0TSR"]
    pub pdma_ch0tsr: PDMA_CH0TSR,
    #[doc = "0x14 - PDMA_CH0CTSR"]
    pub pdma_ch0ctsr: PDMA_CH0CTSR,
    #[doc = "0x18 - PDMA_CH1CR"]
    pub pdma_ch1cr: PDMA_CH1CR,
    #[doc = "0x1c - PDMA_CH1SADR"]
    pub pdma_ch1sadr: PDMA_CH1SADR,
    #[doc = "0x20 - PDMA_CH1DADR"]
    pub pdma_ch1dadr: PDMA_CH1DADR,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - PDMA_CH1TSR"]
    pub pdma_ch1tsr: PDMA_CH1TSR,
    #[doc = "0x2c - PDMA_CH1CTSR"]
    pub pdma_ch1ctsr: PDMA_CH1CTSR,
    #[doc = "0x30 - PDMA_CH2CR"]
    pub pdma_ch2cr: PDMA_CH2CR,
    #[doc = "0x34 - PDMA_CH2SADR"]
    pub pdma_ch2sadr: PDMA_CH2SADR,
    #[doc = "0x38 - PDMA_CH2DADR"]
    pub pdma_ch2dadr: PDMA_CH2DADR,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - PDMA_CH2TSR"]
    pub pdma_ch2tsr: PDMA_CH2TSR,
    #[doc = "0x44 - PDMA_CH2CTSR"]
    pub pdma_ch2ctsr: PDMA_CH2CTSR,
    #[doc = "0x48 - PDMA_CH3CR"]
    pub pdma_ch3cr: PDMA_CH3CR,
    #[doc = "0x4c - PDMA_CH3SADR"]
    pub pdma_ch3sadr: PDMA_CH3SADR,
    #[doc = "0x50 - PDMA_CH3DADR"]
    pub pdma_ch3dadr: PDMA_CH3DADR,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - PDMA_CH3TSR"]
    pub pdma_ch3tsr: PDMA_CH3TSR,
    #[doc = "0x5c - PDMA_CH3CTSR"]
    pub pdma_ch3ctsr: PDMA_CH3CTSR,
    #[doc = "0x60 - PDMA_CH4CR"]
    pub pdma_ch4cr: PDMA_CH4CR,
    #[doc = "0x64 - PDMA_CH4SADR"]
    pub pdma_ch4sadr: PDMA_CH4SADR,
    #[doc = "0x68 - PDMA_CH4DADR"]
    pub pdma_ch4dadr: PDMA_CH4DADR,
    _reserved4: [u8; 4usize],
    #[doc = "0x70 - PDMA_CH4TSR"]
    pub pdma_ch4tsr: PDMA_CH4TSR,
    #[doc = "0x74 - PDMA_CH4CTSR"]
    pub pdma_ch4ctsr: PDMA_CH4CTSR,
    #[doc = "0x78 - PDMA_CH5CR"]
    pub pdma_ch5cr: PDMA_CH5CR,
    #[doc = "0x7c - PDMA_CH5SADR"]
    pub pdma_ch5sadr: PDMA_CH5SADR,
    #[doc = "0x80 - PDMA_CH5DADR"]
    pub pdma_ch5dadr: PDMA_CH5DADR,
    _reserved5: [u8; 4usize],
    #[doc = "0x88 - PDMA_CH5TSR"]
    pub pdma_ch5tsr: PDMA_CH5TSR,
    #[doc = "0x8c - PDMA_CH5CTSR"]
    pub pdma_ch5ctsr: PDMA_CH5CTSR,
    _reserved6: [u8; 144usize],
    #[doc = "0x120 - PDMA_ISR"]
    pub pdma_isr: PDMA_ISR,
    _reserved7: [u8; 4usize],
    #[doc = "0x128 - PDMA_ISCR"]
    pub pdma_iscr: PDMA_ISCR,
    _reserved8: [u8; 4usize],
    #[doc = "0x130 - PDMA_IER"]
    pub pdma_ier: PDMA_IER,
}
#[doc = "PDMA_CH0CR"]
pub struct PDMA_CH0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH0CR"]
pub mod pdma_ch0cr;
#[doc = "PDMA_CH0SADR"]
pub struct PDMA_CH0SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH0SADR"]
pub mod pdma_ch0sadr;
#[doc = "PDMA_CH0DADR"]
pub struct PDMA_CH0DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH0DADR"]
pub mod pdma_ch0dadr;
#[doc = "PDMA_CH0TSR"]
pub struct PDMA_CH0TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH0TSR"]
pub mod pdma_ch0tsr;
#[doc = "PDMA_CH0CTSR"]
pub struct PDMA_CH0CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH0CTSR"]
pub mod pdma_ch0ctsr;
#[doc = "PDMA_CH1CR"]
pub struct PDMA_CH1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH1CR"]
pub mod pdma_ch1cr;
#[doc = "PDMA_CH1SADR"]
pub struct PDMA_CH1SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH1SADR"]
pub mod pdma_ch1sadr;
#[doc = "PDMA_CH1DADR"]
pub struct PDMA_CH1DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH1DADR"]
pub mod pdma_ch1dadr;
#[doc = "PDMA_CH1TSR"]
pub struct PDMA_CH1TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH1TSR"]
pub mod pdma_ch1tsr;
#[doc = "PDMA_CH1CTSR"]
pub struct PDMA_CH1CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH1CTSR"]
pub mod pdma_ch1ctsr;
#[doc = "PDMA_CH2CR"]
pub struct PDMA_CH2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH2CR"]
pub mod pdma_ch2cr;
#[doc = "PDMA_CH2SADR"]
pub struct PDMA_CH2SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH2SADR"]
pub mod pdma_ch2sadr;
#[doc = "PDMA_CH2DADR"]
pub struct PDMA_CH2DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH2DADR"]
pub mod pdma_ch2dadr;
#[doc = "PDMA_CH2TSR"]
pub struct PDMA_CH2TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH2TSR"]
pub mod pdma_ch2tsr;
#[doc = "PDMA_CH2CTSR"]
pub struct PDMA_CH2CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH2CTSR"]
pub mod pdma_ch2ctsr;
#[doc = "PDMA_CH3CR"]
pub struct PDMA_CH3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH3CR"]
pub mod pdma_ch3cr;
#[doc = "PDMA_CH3SADR"]
pub struct PDMA_CH3SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH3SADR"]
pub mod pdma_ch3sadr;
#[doc = "PDMA_CH3DADR"]
pub struct PDMA_CH3DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH3DADR"]
pub mod pdma_ch3dadr;
#[doc = "PDMA_CH3TSR"]
pub struct PDMA_CH3TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH3TSR"]
pub mod pdma_ch3tsr;
#[doc = "PDMA_CH3CTSR"]
pub struct PDMA_CH3CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH3CTSR"]
pub mod pdma_ch3ctsr;
#[doc = "PDMA_CH4CR"]
pub struct PDMA_CH4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH4CR"]
pub mod pdma_ch4cr;
#[doc = "PDMA_CH4SADR"]
pub struct PDMA_CH4SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH4SADR"]
pub mod pdma_ch4sadr;
#[doc = "PDMA_CH4DADR"]
pub struct PDMA_CH4DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH4DADR"]
pub mod pdma_ch4dadr;
#[doc = "PDMA_CH4TSR"]
pub struct PDMA_CH4TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH4TSR"]
pub mod pdma_ch4tsr;
#[doc = "PDMA_CH4CTSR"]
pub struct PDMA_CH4CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH4CTSR"]
pub mod pdma_ch4ctsr;
#[doc = "PDMA_CH5CR"]
pub struct PDMA_CH5CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH5CR"]
pub mod pdma_ch5cr;
#[doc = "PDMA_CH5SADR"]
pub struct PDMA_CH5SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH5SADR"]
pub mod pdma_ch5sadr;
#[doc = "PDMA_CH5DADR"]
pub struct PDMA_CH5DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH5DADR"]
pub mod pdma_ch5dadr;
#[doc = "PDMA_CH5TSR"]
pub struct PDMA_CH5TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH5TSR"]
pub mod pdma_ch5tsr;
#[doc = "PDMA_CH5CTSR"]
pub struct PDMA_CH5CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_CH5CTSR"]
pub mod pdma_ch5ctsr;
#[doc = "PDMA_ISR"]
pub struct PDMA_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_ISR"]
pub mod pdma_isr;
#[doc = "PDMA_ISCR"]
pub struct PDMA_ISCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_ISCR"]
pub mod pdma_iscr;
#[doc = "PDMA_IER"]
pub struct PDMA_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDMA_IER"]
pub mod pdma_ier;
