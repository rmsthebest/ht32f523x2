#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP_CR0"]
    pub cmp_cr0: CMP_CR0,
    #[doc = "0x04 - CMP_VALR0"]
    pub cmp_valr0: CMP_VALR0,
    #[doc = "0x08 - CMP_IER0"]
    pub cmp_ier0: CMP_IER0,
    #[doc = "0x0c - CMP_TFR0"]
    pub cmp_tfr0: CMP_TFR0,
    _reserved0: [u8; 240usize],
    #[doc = "0x100 - CMP_CR1"]
    pub cmp_cr1: CMP_CR1,
    #[doc = "0x104 - CMP_VALR1"]
    pub cmp_valr1: CMP_VALR1,
    #[doc = "0x108 - CMP_IER1"]
    pub cmp_ier1: CMP_IER1,
    #[doc = "0x10c - CMP_TFR1"]
    pub cmp_tfr1: CMP_TFR1,
}
#[doc = "CMP_CR0"]
pub struct CMP_CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_CR0"]
pub mod cmp_cr0;
#[doc = "CMP_VALR0"]
pub struct CMP_VALR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_VALR0"]
pub mod cmp_valr0;
#[doc = "CMP_IER0"]
pub struct CMP_IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_IER0"]
pub mod cmp_ier0;
#[doc = "CMP_TFR0"]
pub struct CMP_TFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_TFR0"]
pub mod cmp_tfr0;
#[doc = "CMP_CR1"]
pub struct CMP_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_CR1"]
pub mod cmp_cr1;
#[doc = "CMP_VALR1"]
pub struct CMP_VALR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_VALR1"]
pub mod cmp_valr1;
#[doc = "CMP_IER1"]
pub struct CMP_IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_IER1"]
pub mod cmp_ier1;
#[doc = "CMP_TFR1"]
pub struct CMP_TFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP_TFR1"]
pub mod cmp_tfr1;
