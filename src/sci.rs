#[doc = r"Register block"]
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
#[doc = "CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "CCR"]
pub mod ccr;
#[doc = "ETUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etur](etur) module"]
pub type ETUR = crate::Reg<u32, _ETUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETUR;
#[doc = "`read()` method returns [etur::R](etur::R) reader structure"]
impl crate::Readable for ETUR {}
#[doc = "`write(|w| ..)` method takes [etur::W](etur::W) writer structure"]
impl crate::Writable for ETUR {}
#[doc = "ETUR"]
pub mod etur;
#[doc = "GTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtr](gtr) module"]
pub type GTR = crate::Reg<u32, _GTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTR;
#[doc = "`read()` method returns [gtr::R](gtr::R) reader structure"]
impl crate::Readable for GTR {}
#[doc = "`write(|w| ..)` method takes [gtr::W](gtr::W) writer structure"]
impl crate::Writable for GTR {}
#[doc = "GTR"]
pub mod gtr;
#[doc = "WTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtr](wtr) module"]
pub type WTR = crate::Reg<u32, _WTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTR;
#[doc = "`read()` method returns [wtr::R](wtr::R) reader structure"]
impl crate::Readable for WTR {}
#[doc = "`write(|w| ..)` method takes [wtr::W](wtr::W) writer structure"]
impl crate::Writable for WTR {}
#[doc = "WTR"]
pub mod wtr;
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
#[doc = "IPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipr](ipr) module"]
pub type IPR = crate::Reg<u32, _IPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPR;
#[doc = "`read()` method returns [ipr::R](ipr::R) reader structure"]
impl crate::Readable for IPR {}
#[doc = "`write(|w| ..)` method takes [ipr::W](ipr::W) writer structure"]
impl crate::Writable for IPR {}
#[doc = "IPR"]
pub mod ipr;
#[doc = "TXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txb](txb) module"]
pub type TXB = crate::Reg<u32, _TXB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXB;
#[doc = "`read()` method returns [txb::R](txb::R) reader structure"]
impl crate::Readable for TXB {}
#[doc = "`write(|w| ..)` method takes [txb::W](txb::W) writer structure"]
impl crate::Writable for TXB {}
#[doc = "TXB"]
pub mod txb;
#[doc = "RXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxb](rxb) module"]
pub type RXB = crate::Reg<u32, _RXB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXB;
#[doc = "`read()` method returns [rxb::R](rxb::R) reader structure"]
impl crate::Readable for RXB {}
#[doc = "`write(|w| ..)` method takes [rxb::W](rxb::W) writer structure"]
impl crate::Writable for RXB {}
#[doc = "RXB"]
pub mod rxb;
#[doc = "PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscr](pscr) module"]
pub type PSCR = crate::Reg<u32, _PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSCR;
#[doc = "`read()` method returns [pscr::R](pscr::R) reader structure"]
impl crate::Readable for PSCR {}
#[doc = "`write(|w| ..)` method takes [pscr::W](pscr::W) writer structure"]
impl crate::Writable for PSCR {}
#[doc = "PSCR"]
pub mod pscr;
