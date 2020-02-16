#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - CDR"]
    pub cdr: CDR,
    #[doc = "0x0c - TXDR"]
    pub txdr: TXDR,
    #[doc = "0x10 - RXDR"]
    pub rxdr: RXDR,
    #[doc = "0x14 - FCR"]
    pub fcr: FCR,
    #[doc = "0x18 - SR"]
    pub sr: SR,
    #[doc = "0x1c - RCNTR"]
    pub rcntr: RCNTR,
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "CR"]
pub mod cr;
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
#[doc = "CDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](cdr) module"]
pub type CDR = crate::Reg<u32, _CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDR;
#[doc = "`read()` method returns [cdr::R](cdr::R) reader structure"]
impl crate::Readable for CDR {}
#[doc = "`write(|w| ..)` method takes [cdr::W](cdr::W) writer structure"]
impl crate::Writable for CDR {}
#[doc = "CDR"]
pub mod cdr;
#[doc = "TXDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr](txdr) module"]
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
#[doc = "`read()` method returns [txdr::R](txdr::R) reader structure"]
impl crate::Readable for TXDR {}
#[doc = "`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure"]
impl crate::Writable for TXDR {}
#[doc = "TXDR"]
pub mod txdr;
#[doc = "RXDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](rxdr) module"]
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
#[doc = "`read()` method returns [rxdr::R](rxdr::R) reader structure"]
impl crate::Readable for RXDR {}
#[doc = "`write(|w| ..)` method takes [rxdr::W](rxdr::W) writer structure"]
impl crate::Writable for RXDR {}
#[doc = "RXDR"]
pub mod rxdr;
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
#[doc = "RCNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcntr](rcntr) module"]
pub type RCNTR = crate::Reg<u32, _RCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCNTR;
#[doc = "`read()` method returns [rcntr::R](rcntr::R) reader structure"]
impl crate::Readable for RCNTR {}
#[doc = "`write(|w| ..)` method takes [rcntr::W](rcntr::W) writer structure"]
impl crate::Writable for RCNTR {}
#[doc = "RCNTR"]
pub mod rcntr;
