#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - SR"]
    pub sr: SR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - ATR"]
    pub atr: ATR,
    #[doc = "0x14 - RTR"]
    pub rtr: RTR,
    #[doc = "0x18 - WTR"]
    pub wtr: WTR,
    #[doc = "0x1c - PR"]
    pub pr: PR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "ATR"]
pub struct ATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ATR"]
pub mod atr;
#[doc = "RTR"]
pub struct RTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTR"]
pub mod rtr;
#[doc = "WTR"]
pub struct WTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WTR"]
pub mod wtr;
#[doc = "PR"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR"]
pub mod pr;
