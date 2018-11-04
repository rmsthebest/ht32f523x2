#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI_CFGR0"]
    pub exti_cfgr0: EXTI_CFGR0,
    #[doc = "0x04 - EXTI_CFGR1"]
    pub exti_cfgr1: EXTI_CFGR1,
    #[doc = "0x08 - EXTI_CFGR2"]
    pub exti_cfgr2: EXTI_CFGR2,
    #[doc = "0x0c - EXTI_CFGR3"]
    pub exti_cfgr3: EXTI_CFGR3,
    #[doc = "0x10 - EXTI_CFGR4"]
    pub exti_cfgr4: EXTI_CFGR4,
    #[doc = "0x14 - EXTI_CFGR5"]
    pub exti_cfgr5: EXTI_CFGR5,
    #[doc = "0x18 - EXTI_CFGR6"]
    pub exti_cfgr6: EXTI_CFGR6,
    #[doc = "0x1c - EXTI_CFGR7"]
    pub exti_cfgr7: EXTI_CFGR7,
    #[doc = "0x20 - EXTI_CFGR8"]
    pub exti_cfgr8: EXTI_CFGR8,
    #[doc = "0x24 - EXTI_CFGR9"]
    pub exti_cfgr9: EXTI_CFGR9,
    #[doc = "0x28 - EXTI_CFGR10"]
    pub exti_cfgr10: EXTI_CFGR10,
    #[doc = "0x2c - EXTI_CFGR11"]
    pub exti_cfgr11: EXTI_CFGR11,
    #[doc = "0x30 - EXTI_CFGR12"]
    pub exti_cfgr12: EXTI_CFGR12,
    #[doc = "0x34 - EXTI_CFGR13"]
    pub exti_cfgr13: EXTI_CFGR13,
    #[doc = "0x38 - EXTI_CFGR14"]
    pub exti_cfgr14: EXTI_CFGR14,
    #[doc = "0x3c - EXTI_CFGR15"]
    pub exti_cfgr15: EXTI_CFGR15,
    #[doc = "0x40 - EXTI_CR"]
    pub exti_cr: EXTI_CR,
    #[doc = "0x44 - EXTI_EDGEFLGR"]
    pub exti_edgeflgr: EXTI_EDGEFLGR,
    #[doc = "0x48 - EXTI_EDGESR"]
    pub exti_edgesr: EXTI_EDGESR,
    #[doc = "0x4c - EXTI_SSCR"]
    pub exti_sscr: EXTI_SSCR,
    #[doc = "0x50 - EXTI_WAKUPCR"]
    pub exti_wakupcr: EXTI_WAKUPCR,
    #[doc = "0x54 - EXTI_WAKUPPOLR"]
    pub exti_wakuppolr: EXTI_WAKUPPOLR,
    #[doc = "0x58 - EXTI_WAKUPFLG"]
    pub exti_wakupflg: EXTI_WAKUPFLG,
}
#[doc = "EXTI_CFGR0"]
pub struct EXTI_CFGR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR0"]
pub mod exti_cfgr0;
#[doc = "EXTI_CFGR1"]
pub struct EXTI_CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR1"]
pub mod exti_cfgr1;
#[doc = "EXTI_CFGR2"]
pub struct EXTI_CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR2"]
pub mod exti_cfgr2;
#[doc = "EXTI_CFGR3"]
pub struct EXTI_CFGR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR3"]
pub mod exti_cfgr3;
#[doc = "EXTI_CFGR4"]
pub struct EXTI_CFGR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR4"]
pub mod exti_cfgr4;
#[doc = "EXTI_CFGR5"]
pub struct EXTI_CFGR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR5"]
pub mod exti_cfgr5;
#[doc = "EXTI_CFGR6"]
pub struct EXTI_CFGR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR6"]
pub mod exti_cfgr6;
#[doc = "EXTI_CFGR7"]
pub struct EXTI_CFGR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR7"]
pub mod exti_cfgr7;
#[doc = "EXTI_CFGR8"]
pub struct EXTI_CFGR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR8"]
pub mod exti_cfgr8;
#[doc = "EXTI_CFGR9"]
pub struct EXTI_CFGR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR9"]
pub mod exti_cfgr9;
#[doc = "EXTI_CFGR10"]
pub struct EXTI_CFGR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR10"]
pub mod exti_cfgr10;
#[doc = "EXTI_CFGR11"]
pub struct EXTI_CFGR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR11"]
pub mod exti_cfgr11;
#[doc = "EXTI_CFGR12"]
pub struct EXTI_CFGR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR12"]
pub mod exti_cfgr12;
#[doc = "EXTI_CFGR13"]
pub struct EXTI_CFGR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR13"]
pub mod exti_cfgr13;
#[doc = "EXTI_CFGR14"]
pub struct EXTI_CFGR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR14"]
pub mod exti_cfgr14;
#[doc = "EXTI_CFGR15"]
pub struct EXTI_CFGR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CFGR15"]
pub mod exti_cfgr15;
#[doc = "EXTI_CR"]
pub struct EXTI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_CR"]
pub mod exti_cr;
#[doc = "EXTI_EDGEFLGR"]
pub struct EXTI_EDGEFLGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_EDGEFLGR"]
pub mod exti_edgeflgr;
#[doc = "EXTI_EDGESR"]
pub struct EXTI_EDGESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_EDGESR"]
pub mod exti_edgesr;
#[doc = "EXTI_SSCR"]
pub struct EXTI_SSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_SSCR"]
pub mod exti_sscr;
#[doc = "EXTI_WAKUPCR"]
pub struct EXTI_WAKUPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_WAKUPCR"]
pub mod exti_wakupcr;
#[doc = "EXTI_WAKUPPOLR"]
pub struct EXTI_WAKUPPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_WAKUPPOLR"]
pub mod exti_wakuppolr;
#[doc = "EXTI_WAKUPFLG"]
pub struct EXTI_WAKUPFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI_WAKUPFLG"]
pub mod exti_wakupflg;
