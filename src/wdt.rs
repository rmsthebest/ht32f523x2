#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - MR0"]
    pub mr0: MR0,
    #[doc = "0x08 - MR1"]
    pub mr1: MR1,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - PR"]
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
#[doc = "MR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr0](mr0) module"]
pub type MR0 = crate::Reg<u32, _MR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR0;
#[doc = "`read()` method returns [mr0::R](mr0::R) reader structure"]
impl crate::Readable for MR0 {}
#[doc = "`write(|w| ..)` method takes [mr0::W](mr0::W) writer structure"]
impl crate::Writable for MR0 {}
#[doc = "MR0"]
pub mod mr0;
#[doc = "MR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr1](mr1) module"]
pub type MR1 = crate::Reg<u32, _MR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR1;
#[doc = "`read()` method returns [mr1::R](mr1::R) reader structure"]
impl crate::Readable for MR1 {}
#[doc = "`write(|w| ..)` method takes [mr1::W](mr1::W) writer structure"]
impl crate::Writable for MR1 {}
#[doc = "MR1"]
pub mod mr1;
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
