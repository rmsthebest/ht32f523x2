#[doc = "Register `GPTM_TRCFR` reader"]
pub struct R(crate::R<GPTM_TRCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_TRCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_TRCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_TRCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_TRCFR` writer"]
pub struct W(crate::W<GPTM_TRCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_TRCFR_SPEC>;
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
impl From<crate::W<GPTM_TRCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_TRCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRSEL` reader - TRSEL"]
pub type TRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRSEL` writer - TRSEL"]
pub type TRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_TRCFR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trsel(&mut self) -> TRSEL_W<0> {
        TRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_trcfr](index.html) module"]
pub struct GPTM_TRCFR_SPEC;
impl crate::RegisterSpec for GPTM_TRCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_trcfr::R](R) reader structure"]
impl crate::Readable for GPTM_TRCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_trcfr::W](W) writer structure"]
impl crate::Writable for GPTM_TRCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_TRCFR to value 0"]
impl crate::Resettable for GPTM_TRCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
