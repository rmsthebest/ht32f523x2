#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ESSR0"]
    pub essr0: ESSR0,
    #[doc = "0x04 - ESSR1"]
    pub essr1: ESSR1,
    _reserved0: [u8; 24usize],
    #[doc = "0x20 - GPACFGLR"]
    pub gpacfglr: GPACFGLR,
    #[doc = "0x24 - GPACFGHR"]
    pub gpacfghr: GPACFGHR,
    #[doc = "0x28 - GPBCFGLR"]
    pub gpbcfglr: GPBCFGLR,
    #[doc = "0x2c - GPBCFGHR"]
    pub gpbcfghr: GPBCFGHR,
    #[doc = "0x30 - GPCCFGLR"]
    pub gpccfglr: GPCCFGLR,
    #[doc = "0x34 - GPCCFGHR"]
    pub gpccfghr: GPCCFGHR,
    #[doc = "0x38 - GPDCFGLR"]
    pub gpdcfglr: GPDCFGLR,
    #[doc = "0x3c - GPDCFGHR"]
    pub gpdcfghr: GPDCFGHR,
}
#[doc = "ESSR0"]
pub struct ESSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ESSR0"]
pub mod essr0;
#[doc = "ESSR1"]
pub struct ESSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ESSR1"]
pub mod essr1;
#[doc = "GPACFGLR"]
pub struct GPACFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPACFGLR"]
pub mod gpacfglr;
#[doc = "GPACFGHR"]
pub struct GPACFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPACFGHR"]
pub mod gpacfghr;
#[doc = "GPBCFGLR"]
pub struct GPBCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPBCFGLR"]
pub mod gpbcfglr;
#[doc = "GPBCFGHR"]
pub struct GPBCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPBCFGHR"]
pub mod gpbcfghr;
#[doc = "GPCCFGLR"]
pub struct GPCCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPCCFGLR"]
pub mod gpccfglr;
#[doc = "GPCCFGHR"]
pub struct GPCCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPCCFGHR"]
pub mod gpccfghr;
#[doc = "GPDCFGLR"]
pub struct GPDCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDCFGLR"]
pub mod gpdcfglr;
#[doc = "GPDCFGHR"]
pub struct GPDCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPDCFGHR"]
pub mod gpdcfghr;
