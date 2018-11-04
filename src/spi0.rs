#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CR0"]
    pub spi_cr0: SPI_CR0,
    #[doc = "0x04 - SPI_CR1"]
    pub spi_cr1: SPI_CR1,
    #[doc = "0x08 - SPI_IER"]
    pub spi_ier: SPI_IER,
    #[doc = "0x0c - SPI_CPR"]
    pub spi_cpr: SPI_CPR,
    #[doc = "0x10 - SPI_DR"]
    pub spi_dr: SPI_DR,
    #[doc = "0x14 - SPI_SR"]
    pub spi_sr: SPI_SR,
    #[doc = "0x18 - SPI_FCR"]
    pub spi_fcr: SPI_FCR,
    #[doc = "0x1c - SPI_FSR"]
    pub spi_fsr: SPI_FSR,
    #[doc = "0x20 - SPI_FTOCR"]
    pub spi_ftocr: SPI_FTOCR,
}
#[doc = "SPI_CR0"]
pub struct SPI_CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_CR0"]
pub mod spi_cr0;
#[doc = "SPI_CR1"]
pub struct SPI_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_CR1"]
pub mod spi_cr1;
#[doc = "SPI_IER"]
pub struct SPI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_IER"]
pub mod spi_ier;
#[doc = "SPI_CPR"]
pub struct SPI_CPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_CPR"]
pub mod spi_cpr;
#[doc = "SPI_DR"]
pub struct SPI_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_DR"]
pub mod spi_dr;
#[doc = "SPI_SR"]
pub struct SPI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_SR"]
pub mod spi_sr;
#[doc = "SPI_FCR"]
pub struct SPI_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_FCR"]
pub mod spi_fcr;
#[doc = "SPI_FSR"]
pub struct SPI_FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_FSR"]
pub mod spi_fsr;
#[doc = "SPI_FTOCR"]
pub struct SPI_FTOCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_FTOCR"]
pub mod spi_ftocr;
