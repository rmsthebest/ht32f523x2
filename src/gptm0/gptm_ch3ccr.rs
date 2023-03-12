#[doc = "Register `GPTM_CH3CCR` reader"]
pub struct R(crate::R<GPTM_CH3CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_CH3CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_CH3CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_CH3CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_CH3CCR` writer"]
pub struct W(crate::W<GPTM_CH3CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_CH3CCR_SPEC>;
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
impl From<crate::W<GPTM_CH3CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_CH3CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3CCV` reader - CH3CCV"]
pub type CH3CCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH3CCV` writer - CH3CCV"]
pub type CH3CCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH3CCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&self) -> CH3CCV_R {
        CH3CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccv(&mut self) -> CH3CCV_W<0> {
        CH3CCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_CH3CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch3ccr](index.html) module"]
pub struct GPTM_CH3CCR_SPEC;
impl crate::RegisterSpec for GPTM_CH3CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_ch3ccr::R](R) reader structure"]
impl crate::Readable for GPTM_CH3CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_ch3ccr::W](W) writer structure"]
impl crate::Writable for GPTM_CH3CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_CH3CCR to value 0"]
impl crate::Resettable for GPTM_CH3CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
