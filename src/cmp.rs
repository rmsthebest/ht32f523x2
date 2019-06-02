#[doc = r" Register block"]
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
    _reserved0: [u8; 240usize],
    #[doc = "0x100 - CR1"]
    pub cr1: CR1,
    #[doc = "0x104 - VALR1"]
    pub valr1: VALR1,
    #[doc = "0x108 - IER1"]
    pub ier1: IER1,
    #[doc = "0x10c - TFR1"]
    pub tfr1: TFR1,
}
#[doc = "CR0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR0"]
pub mod cr0;
#[doc = "VALR0"]
pub struct VALR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VALR0"]
pub mod valr0;
#[doc = "IER0"]
pub struct IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER0"]
pub mod ier0;
#[doc = "TFR0"]
pub struct TFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR0"]
pub mod tfr0;
#[doc = "CR1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR1"]
pub mod cr1;
#[doc = "VALR1"]
pub struct VALR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VALR1"]
pub mod valr1;
#[doc = "IER1"]
pub struct IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER1"]
pub mod ier1;
#[doc = "TFR1"]
pub struct TFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR1"]
pub mod tfr1;
