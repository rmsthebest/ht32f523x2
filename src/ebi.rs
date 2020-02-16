#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - SR"]
    pub sr: SR,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - ATR"]
    pub atr: ATR,
    #[doc = "0x14 - RTR"]
    pub rtr: RTR,
    #[doc = "0x18 - WTR"]
    pub wtr: WTR,
    #[doc = "0x1c - PR"]
    pub pr: PR,
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
#[doc = "ATR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atr](atr) module"]
pub type ATR = crate::Reg<u32, _ATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATR;
#[doc = "`read()` method returns [atr::R](atr::R) reader structure"]
impl crate::Readable for ATR {}
#[doc = "`write(|w| ..)` method takes [atr::W](atr::W) writer structure"]
impl crate::Writable for ATR {}
#[doc = "ATR"]
pub mod atr;
#[doc = "RTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtr](rtr) module"]
pub type RTR = crate::Reg<u32, _RTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTR;
#[doc = "`read()` method returns [rtr::R](rtr::R) reader structure"]
impl crate::Readable for RTR {}
#[doc = "`write(|w| ..)` method takes [rtr::W](rtr::W) writer structure"]
impl crate::Writable for RTR {}
#[doc = "RTR"]
pub mod rtr;
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
#[doc = "PR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "PR"]
pub mod pr;
