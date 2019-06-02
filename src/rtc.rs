#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNT"]
    pub cnt: CNT,
    #[doc = "0x04 - CMP"]
    pub cmp: CMP,
    #[doc = "0x08 - CR"]
    pub cr: CR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - IWEN"]
    pub iwen: IWEN,
}
#[doc = "CNT"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNT"]
pub mod cnt;
#[doc = "CMP"]
pub struct CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMP"]
pub mod cmp;
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
#[doc = "IWEN"]
pub struct IWEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IWEN"]
pub mod iwen;
