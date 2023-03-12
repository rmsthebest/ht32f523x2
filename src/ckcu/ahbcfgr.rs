#[doc = "Register `AHBCFGR` reader"]
pub struct R(crate::R<AHBCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCFGR` writer"]
pub struct W(crate::W<AHBCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHBCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBPRE` reader - AHBPRE"]
pub type AHBPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBPRE` writer - AHBPRE"]
pub type AHBPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&self) -> AHBPRE_R {
        AHBPRE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHBPRE"]
    #[inline(always)]
    #[must_use]
    pub fn ahbpre(&mut self) -> AHBPRE_W<0> {
        AHBPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbcfgr](index.html) module"]
pub struct AHBCFGR_SPEC;
impl crate::RegisterSpec for AHBCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbcfgr::R](R) reader structure"]
impl crate::Readable for AHBCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbcfgr::W](W) writer structure"]
impl crate::Writable for AHBCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBCFGR to value 0"]
impl crate::Resettable for AHBCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
