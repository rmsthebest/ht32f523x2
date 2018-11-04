#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S_CR"]
    pub i2s_cr: I2S_CR,
    #[doc = "0x04 - I2S_IER"]
    pub i2s_ier: I2S_IER,
    #[doc = "0x08 - I2S_CDR"]
    pub i2s_cdr: I2S_CDR,
    #[doc = "0x0c - I2S_TXDR"]
    pub i2s_txdr: I2S_TXDR,
    #[doc = "0x10 - I2S_RXDR"]
    pub i2s_rxdr: I2S_RXDR,
    #[doc = "0x14 - I2S_FCR"]
    pub i2s_fcr: I2S_FCR,
    #[doc = "0x18 - I2S_SR"]
    pub i2s_sr: I2S_SR,
    #[doc = "0x1c - I2S_RCNTR"]
    pub i2s_rcntr: I2S_RCNTR,
}
#[doc = "I2S_CR"]
pub struct I2S_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_CR"]
pub mod i2s_cr;
#[doc = "I2S_IER"]
pub struct I2S_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_IER"]
pub mod i2s_ier;
#[doc = "I2S_CDR"]
pub struct I2S_CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_CDR"]
pub mod i2s_cdr;
#[doc = "I2S_TXDR"]
pub struct I2S_TXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_TXDR"]
pub mod i2s_txdr;
#[doc = "I2S_RXDR"]
pub struct I2S_RXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_RXDR"]
pub mod i2s_rxdr;
#[doc = "I2S_FCR"]
pub struct I2S_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_FCR"]
pub mod i2s_fcr;
#[doc = "I2S_SR"]
pub struct I2S_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_SR"]
pub mod i2s_sr;
#[doc = "I2S_RCNTR"]
pub struct I2S_RCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_RCNTR"]
pub mod i2s_rcntr;
