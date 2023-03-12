#[doc = "Register `GPTM_CH1ICFR` reader"]
pub struct R(crate::R<GPTM_CH1ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_CH1ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_CH1ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_CH1ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_CH1ICFR` writer"]
pub struct W(crate::W<GPTM_CH1ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_CH1ICFR_SPEC>;
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
impl From<crate::W<GPTM_CH1ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_CH1ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1F` reader - TI1F"]
pub type TI1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI1F` writer - TI1F"]
pub type TI1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH1ICFR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH1CCS` reader - CH1CCS"]
pub type CH1CCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1CCS` writer - CH1CCS"]
pub type CH1CCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH1ICFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1PSC` reader - CH1PSC"]
pub type CH1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1PSC` writer - CH1PSC"]
pub type CH1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH1ICFR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    pub fn ti1f(&self) -> TI1F_R {
        TI1F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    pub fn ch1ccs(&self) -> CH1CCS_R {
        CH1CCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    pub fn ch1psc(&self) -> CH1PSC_R {
        CH1PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    #[must_use]
    pub fn ti1f(&mut self) -> TI1F_W<0> {
        TI1F_W::new(self)
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccs(&mut self) -> CH1CCS_W<16> {
        CH1CCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch1psc(&mut self) -> CH1PSC_W<18> {
        CH1PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_CH1ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch1icfr](index.html) module"]
pub struct GPTM_CH1ICFR_SPEC;
impl crate::RegisterSpec for GPTM_CH1ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_ch1icfr::R](R) reader structure"]
impl crate::Readable for GPTM_CH1ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_ch1icfr::W](W) writer structure"]
impl crate::Writable for GPTM_CH1ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_CH1ICFR to value 0"]
impl crate::Resettable for GPTM_CH1ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
