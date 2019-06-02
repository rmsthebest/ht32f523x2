#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - SR"]
    pub sr: SR,
    #[doc = "0x08 - CCR"]
    pub ccr: CCR,
    #[doc = "0x0c - ETUR"]
    pub etur: ETUR,
    #[doc = "0x10 - GTR"]
    pub gtr: GTR,
    #[doc = "0x14 - WTR"]
    pub wtr: WTR,
    #[doc = "0x18 - IER"]
    pub ier: IER,
    #[doc = "0x1c - IPR"]
    pub ipr: IPR,
    #[doc = "0x20 - TXB"]
    pub txb: TXB,
    #[doc = "0x24 - RXB"]
    pub rxb: RXB,
    #[doc = "0x28 - PSCR"]
    pub pscr: PSCR,
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
#[doc = "CCR"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCR"]
pub mod ccr;
#[doc = "ETUR"]
pub struct ETUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETUR"]
pub mod etur;
#[doc = "GTR"]
pub struct GTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTR"]
pub mod gtr;
#[doc = "WTR"]
pub struct WTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WTR"]
pub mod wtr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "IPR"]
pub struct IPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPR"]
pub mod ipr;
#[doc = "TXB"]
pub struct TXB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TXB"]
pub mod txb;
#[doc = "RXB"]
pub struct RXB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXB"]
pub mod rxb;
#[doc = "PSCR"]
pub struct PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSCR"]
pub mod pscr;
