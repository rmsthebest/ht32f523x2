#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC_CR"]
    pub crc_cr: CRC_CR,
    #[doc = "0x04 - CRC_SDR"]
    pub crc_sdr: CRC_SDR,
    #[doc = "0x08 - CRC_CSR"]
    pub crc_csr: CRC_CSR,
    #[doc = "0x0c - CRC_DR"]
    pub crc_dr: CRC_DR,
}
#[doc = "CRC_CR"]
pub struct CRC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC_CR"]
pub mod crc_cr;
#[doc = "CRC_SDR"]
pub struct CRC_SDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC_SDR"]
pub mod crc_sdr;
#[doc = "CRC_CSR"]
pub struct CRC_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC_CSR"]
pub mod crc_csr;
#[doc = "CRC_DR"]
pub struct CRC_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC_DR"]
pub mod crc_dr;
