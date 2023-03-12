#[doc = "Register `GPTM_CNTR` reader"]
pub struct R(crate::R<GPTM_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_CNTR` writer"]
pub struct W(crate::W<GPTM_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_CNTR_SPEC>;
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
impl From<crate::W<GPTM_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTV` reader - CNTV"]
pub type CNTV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNTV` writer - CNTV"]
pub type CNTV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CNTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    pub fn cntv(&self) -> CNTV_R {
        CNTV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    #[must_use]
    pub fn cntv(&mut self) -> CNTV_W<0> {
        CNTV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_cntr](index.html) module"]
pub struct GPTM_CNTR_SPEC;
impl crate::RegisterSpec for GPTM_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_cntr::R](R) reader structure"]
impl crate::Readable for GPTM_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_cntr::W](W) writer structure"]
impl crate::Writable for GPTM_CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_CNTR to value 0"]
impl crate::Resettable for GPTM_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
