#[doc = "Register `GPTM_CH2ICFR` reader"]
pub struct R(crate::R<GPTM_CH2ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_CH2ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_CH2ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_CH2ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_CH2ICFR` writer"]
pub struct W(crate::W<GPTM_CH2ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_CH2ICFR_SPEC>;
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
impl From<crate::W<GPTM_CH2ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_CH2ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI2F` reader - TI2F"]
pub type TI2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI2F` writer - TI2F"]
pub type TI2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH2ICFR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH2CCS` reader - CH2CCS"]
pub type CH2CCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2CCS` writer - CH2CCS"]
pub type CH2CCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH2ICFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH2PSC` reader - CH2PSC"]
pub type CH2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2PSC` writer - CH2PSC"]
pub type CH2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH2ICFR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    pub fn ti2f(&self) -> TI2F_R {
        TI2F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    pub fn ch2ccs(&self) -> CH2CCS_R {
        CH2CCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    pub fn ch2psc(&self) -> CH2PSC_R {
        CH2PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    #[must_use]
    pub fn ti2f(&mut self) -> TI2F_W<0> {
        TI2F_W::new(self)
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccs(&mut self) -> CH2CCS_W<16> {
        CH2CCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch2psc(&mut self) -> CH2PSC_W<18> {
        CH2PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_CH2ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch2icfr](index.html) module"]
pub struct GPTM_CH2ICFR_SPEC;
impl crate::RegisterSpec for GPTM_CH2ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_ch2icfr::R](R) reader structure"]
impl crate::Readable for GPTM_CH2ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_ch2icfr::W](W) writer structure"]
impl crate::Writable for GPTM_CH2ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_CH2ICFR to value 0"]
impl crate::Resettable for GPTM_CH2ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
