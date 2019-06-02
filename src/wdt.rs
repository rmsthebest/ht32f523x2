#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - MR0"]
    pub mr0: MR0,
    #[doc = "0x08 - MR1"]
    pub mr1: MR1,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - PR"]
    pub pr: PR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "MR0"]
pub struct MR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MR0"]
pub mod mr0;
#[doc = "MR1"]
pub struct MR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MR1"]
pub mod mr1;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "PR"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR"]
pub mod pr;
