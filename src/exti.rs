#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGR0"]
    pub cfgr0: CFGR0,
    #[doc = "0x04 - CFGR1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x0c - CFGR3"]
    pub cfgr3: CFGR3,
    #[doc = "0x10 - CFGR4"]
    pub cfgr4: CFGR4,
    #[doc = "0x14 - CFGR5"]
    pub cfgr5: CFGR5,
    #[doc = "0x18 - CFGR6"]
    pub cfgr6: CFGR6,
    #[doc = "0x1c - CFGR7"]
    pub cfgr7: CFGR7,
    #[doc = "0x20 - CFGR8"]
    pub cfgr8: CFGR8,
    #[doc = "0x24 - CFGR9"]
    pub cfgr9: CFGR9,
    #[doc = "0x28 - CFGR10"]
    pub cfgr10: CFGR10,
    #[doc = "0x2c - CFGR11"]
    pub cfgr11: CFGR11,
    #[doc = "0x30 - CFGR12"]
    pub cfgr12: CFGR12,
    #[doc = "0x34 - CFGR13"]
    pub cfgr13: CFGR13,
    #[doc = "0x38 - CFGR14"]
    pub cfgr14: CFGR14,
    #[doc = "0x3c - CFGR15"]
    pub cfgr15: CFGR15,
    #[doc = "0x40 - CR"]
    pub cr: CR,
    #[doc = "0x44 - EDGEFLGR"]
    pub edgeflgr: EDGEFLGR,
    #[doc = "0x48 - EDGESR"]
    pub edgesr: EDGESR,
    #[doc = "0x4c - SSCR"]
    pub sscr: SSCR,
    #[doc = "0x50 - WAKUPCR"]
    pub wakupcr: WAKUPCR,
    #[doc = "0x54 - WAKUPPOLR"]
    pub wakuppolr: WAKUPPOLR,
    #[doc = "0x58 - WAKUPFLG"]
    pub wakupflg: WAKUPFLG,
}
#[doc = "CFGR0"]
pub struct CFGR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR0"]
pub mod cfgr0;
#[doc = "CFGR1"]
pub struct CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2"]
pub struct CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3"]
pub struct CFGR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4"]
pub struct CFGR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR4"]
pub mod cfgr4;
#[doc = "CFGR5"]
pub struct CFGR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR5"]
pub mod cfgr5;
#[doc = "CFGR6"]
pub struct CFGR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR6"]
pub mod cfgr6;
#[doc = "CFGR7"]
pub struct CFGR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR7"]
pub mod cfgr7;
#[doc = "CFGR8"]
pub struct CFGR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR8"]
pub mod cfgr8;
#[doc = "CFGR9"]
pub struct CFGR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR9"]
pub mod cfgr9;
#[doc = "CFGR10"]
pub struct CFGR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR10"]
pub mod cfgr10;
#[doc = "CFGR11"]
pub struct CFGR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR11"]
pub mod cfgr11;
#[doc = "CFGR12"]
pub struct CFGR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR12"]
pub mod cfgr12;
#[doc = "CFGR13"]
pub struct CFGR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR13"]
pub mod cfgr13;
#[doc = "CFGR14"]
pub struct CFGR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR14"]
pub mod cfgr14;
#[doc = "CFGR15"]
pub struct CFGR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR15"]
pub mod cfgr15;
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "EDGEFLGR"]
pub struct EDGEFLGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EDGEFLGR"]
pub mod edgeflgr;
#[doc = "EDGESR"]
pub struct EDGESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EDGESR"]
pub mod edgesr;
#[doc = "SSCR"]
pub struct SSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSCR"]
pub mod sscr;
#[doc = "WAKUPCR"]
pub struct WAKUPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WAKUPCR"]
pub mod wakupcr;
#[doc = "WAKUPPOLR"]
pub struct WAKUPPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WAKUPPOLR"]
pub mod wakuppolr;
#[doc = "WAKUPFLG"]
pub struct WAKUPFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WAKUPFLG"]
pub mod wakupflg;
