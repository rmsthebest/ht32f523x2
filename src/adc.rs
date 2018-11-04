#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_CR"]
    pub adc_cr: ADC_CR,
    #[doc = "0x04 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x08 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - ADC_STR"]
    pub adc_str: ADC_STR,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - ADC_DR0"]
    pub adc_dr0: ADC_DR0,
    #[doc = "0x34 - ADC_DR1"]
    pub adc_dr1: ADC_DR1,
    #[doc = "0x38 - ADC_DR2"]
    pub adc_dr2: ADC_DR2,
    #[doc = "0x3c - ADC_DR3"]
    pub adc_dr3: ADC_DR3,
    #[doc = "0x40 - ADC_DR4"]
    pub adc_dr4: ADC_DR4,
    #[doc = "0x44 - ADC_DR5"]
    pub adc_dr5: ADC_DR5,
    #[doc = "0x48 - ADC_DR6"]
    pub adc_dr6: ADC_DR6,
    #[doc = "0x4c - ADC_DR7"]
    pub adc_dr7: ADC_DR7,
    _reserved2: [u8; 32usize],
    #[doc = "0x70 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x74 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    #[doc = "0x78 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x7c - ADC_TR"]
    pub adc_tr: ADC_TR,
    #[doc = "0x80 - ADC_IMR"]
    pub adc_ier: ADC_IER,
    #[doc = "0x84 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x88 - ADC_ISR"]
    pub adc_isr: ADC_ISR,
    #[doc = "0x8c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
    #[doc = "0x90 - ADC_DMAR"]
    pub adc_dmar: ADC_DMAR,
}
#[doc = "ADC_CR"]
pub struct ADC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_CR"]
pub mod adc_cr;
#[doc = "ADC_LST0"]
pub struct ADC_LST0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1"]
pub struct ADC_LST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_STR"]
pub struct ADC_STR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_STR"]
pub mod adc_str;
#[doc = "ADC_DR0"]
pub struct ADC_DR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR0"]
pub mod adc_dr0;
#[doc = "ADC_DR1"]
pub struct ADC_DR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR1"]
pub mod adc_dr1;
#[doc = "ADC_DR2"]
pub struct ADC_DR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR2"]
pub mod adc_dr2;
#[doc = "ADC_DR3"]
pub struct ADC_DR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR3"]
pub mod adc_dr3;
#[doc = "ADC_DR4"]
pub struct ADC_DR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR4"]
pub mod adc_dr4;
#[doc = "ADC_DR5"]
pub struct ADC_DR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR5"]
pub mod adc_dr5;
#[doc = "ADC_DR6"]
pub struct ADC_DR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR6"]
pub mod adc_dr6;
#[doc = "ADC_DR7"]
pub struct ADC_DR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DR7"]
pub mod adc_dr7;
#[doc = "ADC_TCR"]
pub struct ADC_TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_TCR"]
pub mod adc_tcr;
#[doc = "ADC_TSR"]
pub struct ADC_TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_TSR"]
pub mod adc_tsr;
#[doc = "ADC_WCR"]
pub struct ADC_WCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_WCR"]
pub mod adc_wcr;
#[doc = "ADC_TR"]
pub struct ADC_TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_TR"]
pub mod adc_tr;
#[doc = "ADC_IMR"]
pub struct ADC_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_IMR"]
pub mod adc_ier;
#[doc = "ADC_IRAW"]
pub struct ADC_IRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_IRAW"]
pub mod adc_iraw;
#[doc = "ADC_ISR"]
pub struct ADC_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_ISR"]
pub mod adc_isr;
#[doc = "ADC_ICLR"]
pub struct ADC_ICLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_ICLR"]
pub mod adc_iclr;
#[doc = "ADC_DMAR"]
pub struct ADC_DMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_DMAR"]
pub mod adc_dmar;
