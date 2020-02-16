#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GRSR"]
    pub grsr: GRSR,
    #[doc = "0x04 - AHBPRSTR"]
    pub ahbprstr: AHBPRSTR,
    #[doc = "0x08 - APBPRSTR0"]
    pub apbprstr0: APBPRSTR0,
    #[doc = "0x0c - APBPRSTR1"]
    pub apbprstr1: APBPRSTR1,
}
#[doc = "GRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grsr](grsr) module"]
pub type GRSR = crate::Reg<u32, _GRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSR;
#[doc = "`read()` method returns [grsr::R](grsr::R) reader structure"]
impl crate::Readable for GRSR {}
#[doc = "`write(|w| ..)` method takes [grsr::W](grsr::W) writer structure"]
impl crate::Writable for GRSR {}
#[doc = "GRSR"]
pub mod grsr;
#[doc = "AHBPRSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbprstr](ahbprstr) module"]
pub type AHBPRSTR = crate::Reg<u32, _AHBPRSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBPRSTR;
#[doc = "`read()` method returns [ahbprstr::R](ahbprstr::R) reader structure"]
impl crate::Readable for AHBPRSTR {}
#[doc = "`write(|w| ..)` method takes [ahbprstr::W](ahbprstr::W) writer structure"]
impl crate::Writable for AHBPRSTR {}
#[doc = "AHBPRSTR"]
pub mod ahbprstr;
#[doc = "APBPRSTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbprstr0](apbprstr0) module"]
pub type APBPRSTR0 = crate::Reg<u32, _APBPRSTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBPRSTR0;
#[doc = "`read()` method returns [apbprstr0::R](apbprstr0::R) reader structure"]
impl crate::Readable for APBPRSTR0 {}
#[doc = "`write(|w| ..)` method takes [apbprstr0::W](apbprstr0::W) writer structure"]
impl crate::Writable for APBPRSTR0 {}
#[doc = "APBPRSTR0"]
pub mod apbprstr0;
#[doc = "APBPRSTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbprstr1](apbprstr1) module"]
pub type APBPRSTR1 = crate::Reg<u32, _APBPRSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBPRSTR1;
#[doc = "`read()` method returns [apbprstr1::R](apbprstr1::R) reader structure"]
impl crate::Readable for APBPRSTR1 {}
#[doc = "`write(|w| ..)` method takes [apbprstr1::W](apbprstr1::W) writer structure"]
impl crate::Writable for APBPRSTR1 {}
#[doc = "APBPRSTR1"]
pub mod apbprstr1;
