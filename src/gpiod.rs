#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DIRCR"]
    pub dircr: DIRCR,
    #[doc = "0x04 - INER"]
    pub iner: INER,
    #[doc = "0x08 - PUR"]
    pub pur: PUR,
    #[doc = "0x0c - PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - ODR"]
    pub odr: ODR,
    #[doc = "0x14 - DRVR"]
    pub drvr: DRVR,
    #[doc = "0x18 - LOCKR"]
    pub lockr: LOCKR,
    #[doc = "0x1c - DINR"]
    pub dinr: DINR,
    #[doc = "0x20 - DOUTR"]
    pub doutr: DOUTR,
    #[doc = "0x24 - SRR"]
    pub srr: SRR,
    #[doc = "0x28 - RR"]
    pub rr: RR,
}
#[doc = "DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dircr](dircr) module"]
pub type DIRCR = crate::Reg<u32, _DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRCR;
#[doc = "`read()` method returns [dircr::R](dircr::R) reader structure"]
impl crate::Readable for DIRCR {}
#[doc = "`write(|w| ..)` method takes [dircr::W](dircr::W) writer structure"]
impl crate::Writable for DIRCR {}
#[doc = "DIRCR"]
pub mod dircr;
#[doc = "INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iner](iner) module"]
pub type INER = crate::Reg<u32, _INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INER;
#[doc = "`read()` method returns [iner::R](iner::R) reader structure"]
impl crate::Readable for INER {}
#[doc = "`write(|w| ..)` method takes [iner::W](iner::W) writer structure"]
impl crate::Writable for INER {}
#[doc = "INER"]
pub mod iner;
#[doc = "PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pur](pur) module"]
pub type PUR = crate::Reg<u32, _PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUR;
#[doc = "`read()` method returns [pur::R](pur::R) reader structure"]
impl crate::Readable for PUR {}
#[doc = "`write(|w| ..)` method takes [pur::W](pur::W) writer structure"]
impl crate::Writable for PUR {}
#[doc = "PUR"]
pub mod pur;
#[doc = "PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr](pdr) module"]
pub type PDR = crate::Reg<u32, _PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR;
#[doc = "`read()` method returns [pdr::R](pdr::R) reader structure"]
impl crate::Readable for PDR {}
#[doc = "`write(|w| ..)` method takes [pdr::W](pdr::W) writer structure"]
impl crate::Writable for PDR {}
#[doc = "PDR"]
pub mod pdr;
#[doc = "ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](odr) module"]
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
#[doc = "`read()` method returns [odr::R](odr::R) reader structure"]
impl crate::Readable for ODR {}
#[doc = "`write(|w| ..)` method takes [odr::W](odr::W) writer structure"]
impl crate::Writable for ODR {}
#[doc = "ODR"]
pub mod odr;
#[doc = "DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvr](drvr) module"]
pub type DRVR = crate::Reg<u32, _DRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRVR;
#[doc = "`read()` method returns [drvr::R](drvr::R) reader structure"]
impl crate::Readable for DRVR {}
#[doc = "`write(|w| ..)` method takes [drvr::W](drvr::W) writer structure"]
impl crate::Writable for DRVR {}
#[doc = "DRVR"]
pub mod drvr;
#[doc = "LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockr](lockr) module"]
pub type LOCKR = crate::Reg<u32, _LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKR;
#[doc = "`read()` method returns [lockr::R](lockr::R) reader structure"]
impl crate::Readable for LOCKR {}
#[doc = "`write(|w| ..)` method takes [lockr::W](lockr::W) writer structure"]
impl crate::Writable for LOCKR {}
#[doc = "LOCKR"]
pub mod lockr;
#[doc = "DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](dinr) module"]
pub type DINR = crate::Reg<u32, _DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR;
#[doc = "`read()` method returns [dinr::R](dinr::R) reader structure"]
impl crate::Readable for DINR {}
#[doc = "`write(|w| ..)` method takes [dinr::W](dinr::W) writer structure"]
impl crate::Writable for DINR {}
#[doc = "DINR"]
pub mod dinr;
#[doc = "DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr](doutr) module"]
pub type DOUTR = crate::Reg<u32, _DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR;
#[doc = "`read()` method returns [doutr::R](doutr::R) reader structure"]
impl crate::Readable for DOUTR {}
#[doc = "`write(|w| ..)` method takes [doutr::W](doutr::W) writer structure"]
impl crate::Writable for DOUTR {}
#[doc = "DOUTR"]
pub mod doutr;
#[doc = "SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](srr) module"]
pub type SRR = crate::Reg<u32, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "`write(|w| ..)` method takes [srr::W](srr::W) writer structure"]
impl crate::Writable for SRR {}
#[doc = "SRR"]
pub mod srr;
#[doc = "RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](rr) module"]
pub type RR = crate::Reg<u32, _RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RR;
#[doc = "`read()` method returns [rr::R](rr::R) reader structure"]
impl crate::Readable for RR {}
#[doc = "`write(|w| ..)` method takes [rr::W](rr::W) writer structure"]
impl crate::Writable for RR {}
#[doc = "RR"]
pub mod rr;
