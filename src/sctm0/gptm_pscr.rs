#[doc = "Register `GPTM_PSCR` reader"]
pub struct R(crate::R<GPTM_PSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_PSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_PSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_PSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_PSCR` writer"]
pub struct W(crate::W<GPTM_PSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_PSCR_SPEC>;
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
impl From<crate::W<GPTM_PSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_PSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSCV` reader - PSCV"]
pub type PSCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSCV` writer - PSCV"]
pub type PSCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_PSCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    pub fn pscv(&self) -> PSCV_R {
        PSCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    #[must_use]
    pub fn pscv(&mut self) -> PSCV_W<0> {
        PSCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_pscr](index.html) module"]
pub struct GPTM_PSCR_SPEC;
impl crate::RegisterSpec for GPTM_PSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_pscr::R](R) reader structure"]
impl crate::Readable for GPTM_PSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_pscr::W](W) writer structure"]
impl crate::Writable for GPTM_PSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_PSCR to value 0"]
impl crate::Resettable for GPTM_PSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
