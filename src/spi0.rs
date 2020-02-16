#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR0"]
    pub cr0: CR0,
    #[doc = "0x04 - CR1"]
    pub cr1: CR1,
    #[doc = "0x08 - IER"]
    pub ier: IER,
    #[doc = "0x0c - CPR"]
    pub cpr: CPR,
    #[doc = "0x10 - DR"]
    pub dr: DR,
    #[doc = "0x14 - SR"]
    pub sr: SR,
    #[doc = "0x18 - FCR"]
    pub fcr: FCR,
    #[doc = "0x1c - FSR"]
    pub fsr: FSR,
    #[doc = "0x20 - FTOCR"]
    pub ftocr: FTOCR,
}
#[doc = "CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u32, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "CR0"]
pub mod cr0;
#[doc = "CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "CR1"]
pub mod cr1;
#[doc = "IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "IER"]
pub mod ier;
#[doc = "CPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpr](cpr) module"]
pub type CPR = crate::Reg<u32, _CPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPR;
#[doc = "`read()` method returns [cpr::R](cpr::R) reader structure"]
impl crate::Readable for CPR {}
#[doc = "`write(|w| ..)` method takes [cpr::W](cpr::W) writer structure"]
impl crate::Writable for CPR {}
#[doc = "CPR"]
pub mod cpr;
#[doc = "DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "DR"]
pub mod dr;
#[doc = "SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "SR"]
pub mod sr;
#[doc = "FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "FCR"]
pub mod fcr;
#[doc = "FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](fsr) module"]
pub type FSR = crate::Reg<u32, _FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSR;
#[doc = "`read()` method returns [fsr::R](fsr::R) reader structure"]
impl crate::Readable for FSR {}
#[doc = "`write(|w| ..)` method takes [fsr::W](fsr::W) writer structure"]
impl crate::Writable for FSR {}
#[doc = "FSR"]
pub mod fsr;
#[doc = "FTOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftocr](ftocr) module"]
pub type FTOCR = crate::Reg<u32, _FTOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTOCR;
#[doc = "`read()` method returns [ftocr::R](ftocr::R) reader structure"]
impl crate::Readable for FTOCR {}
#[doc = "`write(|w| ..)` method takes [ftocr::W](ftocr::W) writer structure"]
impl crate::Writable for FTOCR {}
#[doc = "FTOCR"]
pub mod ftocr;
