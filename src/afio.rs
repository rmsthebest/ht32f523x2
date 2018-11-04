#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ESSR0"]
    pub afio_essr0: AFIO_ESSR0,
    #[doc = "0x04 - AFIO_ESSR1"]
    pub afio_essr1: AFIO_ESSR1,
    _reserved0: [u8; 24usize],
    #[doc = "0x20 - AFIO_GPACFGLR"]
    pub afio_gpacfglr: AFIO_GPACFGLR,
    #[doc = "0x24 - AFIO_GPACFGHR"]
    pub afio_gpacfghr: AFIO_GPACFGHR,
    #[doc = "0x28 - AFIO_GPBCFGLR"]
    pub afio_gpbcfglr: AFIO_GPBCFGLR,
    #[doc = "0x2c - AFIO_GPBCFGHR"]
    pub afio_gpbcfghr: AFIO_GPBCFGHR,
    #[doc = "0x30 - AFIO_GPCCFGLR"]
    pub afio_gpccfglr: AFIO_GPCCFGLR,
    #[doc = "0x34 - AFIO_GPCCFGHR"]
    pub afio_gpccfghr: AFIO_GPCCFGHR,
    #[doc = "0x38 - AFIO_GPDCFGLR"]
    pub afio_gpdcfglr: AFIO_GPDCFGLR,
    #[doc = "0x3c - AFIO_GPDCFGHR"]
    pub afio_gpdcfghr: AFIO_GPDCFGHR,
}
#[doc = "AFIO_ESSR0"]
pub struct AFIO_ESSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_ESSR0"]
pub mod afio_essr0;
#[doc = "AFIO_ESSR1"]
pub struct AFIO_ESSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_ESSR1"]
pub mod afio_essr1;
#[doc = "AFIO_GPACFGLR"]
pub struct AFIO_GPACFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPACFGLR"]
pub mod afio_gpacfglr;
#[doc = "AFIO_GPACFGHR"]
pub struct AFIO_GPACFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPACFGHR"]
pub mod afio_gpacfghr;
#[doc = "AFIO_GPBCFGLR"]
pub struct AFIO_GPBCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPBCFGLR"]
pub mod afio_gpbcfglr;
#[doc = "AFIO_GPBCFGHR"]
pub struct AFIO_GPBCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPBCFGHR"]
pub mod afio_gpbcfghr;
#[doc = "AFIO_GPCCFGLR"]
pub struct AFIO_GPCCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPCCFGLR"]
pub mod afio_gpccfglr;
#[doc = "AFIO_GPCCFGHR"]
pub struct AFIO_GPCCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPCCFGHR"]
pub mod afio_gpccfghr;
#[doc = "AFIO_GPDCFGLR"]
pub struct AFIO_GPDCFGLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPDCFGLR"]
pub mod afio_gpdcfglr;
#[doc = "AFIO_GPDCFGHR"]
pub struct AFIO_GPDCFGHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO_GPDCFGHR"]
pub mod afio_gpdcfghr;
