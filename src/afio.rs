#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ESSR0"]
    pub essr0: ESSR0,
    #[doc = "0x04 - ESSR1"]
    pub essr1: ESSR1,
    _reserved2: [u8; 0x18],
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
#[doc = "ESSR0 (rw) register accessor: an alias for `Reg<ESSR0_SPEC>`"]
pub type ESSR0 = crate::Reg<essr0::ESSR0_SPEC>;
#[doc = "ESSR0"]
pub mod essr0;
#[doc = "ESSR1 (rw) register accessor: an alias for `Reg<ESSR1_SPEC>`"]
pub type ESSR1 = crate::Reg<essr1::ESSR1_SPEC>;
#[doc = "ESSR1"]
pub mod essr1;
#[doc = "GPACFGLR (rw) register accessor: an alias for `Reg<GPACFGLR_SPEC>`"]
pub type GPACFGLR = crate::Reg<gpacfglr::GPACFGLR_SPEC>;
#[doc = "GPACFGLR"]
pub mod gpacfglr;
#[doc = "GPACFGHR (rw) register accessor: an alias for `Reg<GPACFGHR_SPEC>`"]
pub type GPACFGHR = crate::Reg<gpacfghr::GPACFGHR_SPEC>;
#[doc = "GPACFGHR"]
pub mod gpacfghr;
#[doc = "GPBCFGLR (rw) register accessor: an alias for `Reg<GPBCFGLR_SPEC>`"]
pub type GPBCFGLR = crate::Reg<gpbcfglr::GPBCFGLR_SPEC>;
#[doc = "GPBCFGLR"]
pub mod gpbcfglr;
#[doc = "GPBCFGHR (rw) register accessor: an alias for `Reg<GPBCFGHR_SPEC>`"]
pub type GPBCFGHR = crate::Reg<gpbcfghr::GPBCFGHR_SPEC>;
#[doc = "GPBCFGHR"]
pub mod gpbcfghr;
#[doc = "GPCCFGLR (rw) register accessor: an alias for `Reg<GPCCFGLR_SPEC>`"]
pub type GPCCFGLR = crate::Reg<gpccfglr::GPCCFGLR_SPEC>;
#[doc = "GPCCFGLR"]
pub mod gpccfglr;
#[doc = "GPCCFGHR (rw) register accessor: an alias for `Reg<GPCCFGHR_SPEC>`"]
pub type GPCCFGHR = crate::Reg<gpccfghr::GPCCFGHR_SPEC>;
#[doc = "GPCCFGHR"]
pub mod gpccfghr;
#[doc = "GPDCFGLR (rw) register accessor: an alias for `Reg<GPDCFGLR_SPEC>`"]
pub type GPDCFGLR = crate::Reg<gpdcfglr::GPDCFGLR_SPEC>;
#[doc = "GPDCFGLR"]
pub mod gpdcfglr;
#[doc = "GPDCFGHR (rw) register accessor: an alias for `Reg<GPDCFGHR_SPEC>`"]
pub type GPDCFGHR = crate::Reg<gpdcfghr::GPDCFGHR_SPEC>;
#[doc = "GPDCFGHR"]
pub mod gpdcfghr;
