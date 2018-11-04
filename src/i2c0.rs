#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_CR"]
    pub i2c_cr: I2C_CR,
    #[doc = "0x04 - I2C_IER"]
    pub i2c_ier: I2C_IER,
    #[doc = "0x08 - I2C_ADDR"]
    pub i2c_addr: I2C_ADDR,
    #[doc = "0x0c - I2C_SR"]
    pub i2c_sr: I2C_SR,
    #[doc = "0x10 - I2C_SHPGR"]
    pub i2c_shpgr: I2C_SHPGR,
    #[doc = "0x14 - I2C_SLPGR"]
    pub i2c_slpgr: I2C_SLPGR,
    #[doc = "0x18 - I2C_DR"]
    pub i2c_dr: I2C_DR,
    #[doc = "0x1c - I2C_TAR"]
    pub i2c_tar: I2C_TAR,
    #[doc = "0x20 - I2C_ADDMR"]
    pub i2c_addmr: I2C_ADDMR,
    #[doc = "0x24 - I2C_ADDSR"]
    pub i2c_addsr: I2C_ADDSR,
    #[doc = "0x28 - I2C_TOUT"]
    pub i2c_tout: I2C_TOUT,
}
#[doc = "I2C_CR"]
pub struct I2C_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_CR"]
pub mod i2c_cr;
#[doc = "I2C_IER"]
pub struct I2C_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_IER"]
pub mod i2c_ier;
#[doc = "I2C_ADDR"]
pub struct I2C_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_ADDR"]
pub mod i2c_addr;
#[doc = "I2C_SR"]
pub struct I2C_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_SR"]
pub mod i2c_sr;
#[doc = "I2C_SHPGR"]
pub struct I2C_SHPGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_SHPGR"]
pub mod i2c_shpgr;
#[doc = "I2C_SLPGR"]
pub struct I2C_SLPGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_SLPGR"]
pub mod i2c_slpgr;
#[doc = "I2C_DR"]
pub struct I2C_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_DR"]
pub mod i2c_dr;
#[doc = "I2C_TAR"]
pub struct I2C_TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_TAR"]
pub mod i2c_tar;
#[doc = "I2C_ADDMR"]
pub struct I2C_ADDMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_ADDMR"]
pub mod i2c_addmr;
#[doc = "I2C_ADDSR"]
pub struct I2C_ADDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_ADDSR"]
pub mod i2c_addsr;
#[doc = "I2C_TOUT"]
pub struct I2C_TOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_TOUT"]
pub mod i2c_tout;
