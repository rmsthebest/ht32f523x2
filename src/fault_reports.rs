#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSR"]
    pub dfsr: DFSR,
}
#[doc = "DFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](dfsr) module"]
pub type DFSR = crate::Reg<u32, _DFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSR;
#[doc = "`read()` method returns [dfsr::R](dfsr::R) reader structure"]
impl crate::Readable for DFSR {}
#[doc = "`write(|w| ..)` method takes [dfsr::W](dfsr::W) writer structure"]
impl crate::Writable for DFSR {}
#[doc = "DFSR"]
pub mod dfsr;
