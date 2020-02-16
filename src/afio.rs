#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ESSR0"]
    pub essr0: ESSR0,
    #[doc = "0x04 - ESSR1"]
    pub essr1: ESSR1,
    _reserved2: [u8; 24usize],
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
#[doc = "ESSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [essr0](essr0) module"]
pub type ESSR0 = crate::Reg<u32, _ESSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESSR0;
#[doc = "`read()` method returns [essr0::R](essr0::R) reader structure"]
impl crate::Readable for ESSR0 {}
#[doc = "`write(|w| ..)` method takes [essr0::W](essr0::W) writer structure"]
impl crate::Writable for ESSR0 {}
#[doc = "ESSR0"]
pub mod essr0;
#[doc = "ESSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [essr1](essr1) module"]
pub type ESSR1 = crate::Reg<u32, _ESSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESSR1;
#[doc = "`read()` method returns [essr1::R](essr1::R) reader structure"]
impl crate::Readable for ESSR1 {}
#[doc = "`write(|w| ..)` method takes [essr1::W](essr1::W) writer structure"]
impl crate::Writable for ESSR1 {}
#[doc = "ESSR1"]
pub mod essr1;
#[doc = "GPACFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpacfglr](gpacfglr) module"]
pub type GPACFGLR = crate::Reg<u32, _GPACFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPACFGLR;
#[doc = "`read()` method returns [gpacfglr::R](gpacfglr::R) reader structure"]
impl crate::Readable for GPACFGLR {}
#[doc = "`write(|w| ..)` method takes [gpacfglr::W](gpacfglr::W) writer structure"]
impl crate::Writable for GPACFGLR {}
#[doc = "GPACFGLR"]
pub mod gpacfglr;
#[doc = "GPACFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpacfghr](gpacfghr) module"]
pub type GPACFGHR = crate::Reg<u32, _GPACFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPACFGHR;
#[doc = "`read()` method returns [gpacfghr::R](gpacfghr::R) reader structure"]
impl crate::Readable for GPACFGHR {}
#[doc = "`write(|w| ..)` method takes [gpacfghr::W](gpacfghr::W) writer structure"]
impl crate::Writable for GPACFGHR {}
#[doc = "GPACFGHR"]
pub mod gpacfghr;
#[doc = "GPBCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpbcfglr](gpbcfglr) module"]
pub type GPBCFGLR = crate::Reg<u32, _GPBCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPBCFGLR;
#[doc = "`read()` method returns [gpbcfglr::R](gpbcfglr::R) reader structure"]
impl crate::Readable for GPBCFGLR {}
#[doc = "`write(|w| ..)` method takes [gpbcfglr::W](gpbcfglr::W) writer structure"]
impl crate::Writable for GPBCFGLR {}
#[doc = "GPBCFGLR"]
pub mod gpbcfglr;
#[doc = "GPBCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpbcfghr](gpbcfghr) module"]
pub type GPBCFGHR = crate::Reg<u32, _GPBCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPBCFGHR;
#[doc = "`read()` method returns [gpbcfghr::R](gpbcfghr::R) reader structure"]
impl crate::Readable for GPBCFGHR {}
#[doc = "`write(|w| ..)` method takes [gpbcfghr::W](gpbcfghr::W) writer structure"]
impl crate::Writable for GPBCFGHR {}
#[doc = "GPBCFGHR"]
pub mod gpbcfghr;
#[doc = "GPCCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpccfglr](gpccfglr) module"]
pub type GPCCFGLR = crate::Reg<u32, _GPCCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCCFGLR;
#[doc = "`read()` method returns [gpccfglr::R](gpccfglr::R) reader structure"]
impl crate::Readable for GPCCFGLR {}
#[doc = "`write(|w| ..)` method takes [gpccfglr::W](gpccfglr::W) writer structure"]
impl crate::Writable for GPCCFGLR {}
#[doc = "GPCCFGLR"]
pub mod gpccfglr;
#[doc = "GPCCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpccfghr](gpccfghr) module"]
pub type GPCCFGHR = crate::Reg<u32, _GPCCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCCFGHR;
#[doc = "`read()` method returns [gpccfghr::R](gpccfghr::R) reader structure"]
impl crate::Readable for GPCCFGHR {}
#[doc = "`write(|w| ..)` method takes [gpccfghr::W](gpccfghr::W) writer structure"]
impl crate::Writable for GPCCFGHR {}
#[doc = "GPCCFGHR"]
pub mod gpccfghr;
#[doc = "GPDCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdcfglr](gpdcfglr) module"]
pub type GPDCFGLR = crate::Reg<u32, _GPDCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDCFGLR;
#[doc = "`read()` method returns [gpdcfglr::R](gpdcfglr::R) reader structure"]
impl crate::Readable for GPDCFGLR {}
#[doc = "`write(|w| ..)` method takes [gpdcfglr::W](gpdcfglr::W) writer structure"]
impl crate::Writable for GPDCFGLR {}
#[doc = "GPDCFGLR"]
pub mod gpdcfglr;
#[doc = "GPDCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdcfghr](gpdcfghr) module"]
pub type GPDCFGHR = crate::Reg<u32, _GPDCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDCFGHR;
#[doc = "`read()` method returns [gpdcfghr::R](gpdcfghr::R) reader structure"]
impl crate::Readable for GPDCFGHR {}
#[doc = "`write(|w| ..)` method takes [gpdcfghr::W](gpdcfghr::W) writer structure"]
impl crate::Writable for GPDCFGHR {}
#[doc = "GPDCFGHR"]
pub mod gpdcfghr;
