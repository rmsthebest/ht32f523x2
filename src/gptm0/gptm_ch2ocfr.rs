#[doc = "Register `GPTM_CH2OCFR` reader"]
pub struct R(crate::R<GPTM_CH2OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_CH2OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_CH2OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_CH2OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_CH2OCFR` writer"]
pub struct W(crate::W<GPTM_CH2OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_CH2OCFR_SPEC>;
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
impl From<crate::W<GPTM_CH2OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_CH2OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2OM` reader - CH2OM"]
pub type CH2OM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2OM` writer - CH2OM"]
pub type CH2OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTM_CH2OCFR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CH2PRE` reader - CH2PRE"]
pub type CH2PRE_R = crate::BitReader<bool>;
#[doc = "Field `CH2PRE` writer - CH2PRE"]
pub type CH2PRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_CH2OCFR_SPEC, bool, O>;
#[doc = "Field `CH2IMAE` reader - CH2IMAE"]
pub type CH2IMAE_R = crate::BitReader<bool>;
#[doc = "Field `CH2IMAE` writer - CH2IMAE"]
pub type CH2IMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_CH2OCFR_SPEC, bool, O>;
#[doc = "Field `CH2OM3` reader - CH2OM3"]
pub type CH2OM3_R = crate::BitReader<bool>;
#[doc = "Field `CH2OM3` writer - CH2OM3"]
pub type CH2OM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_CH2OCFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    pub fn ch2pre(&self) -> CH2PRE_R {
        CH2PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    pub fn ch2imae(&self) -> CH2IMAE_R {
        CH2IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    pub fn ch2om3(&self) -> CH2OM3_R {
        CH2OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<0> {
        CH2OM_W::new(self)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pre(&mut self) -> CH2PRE_W<4> {
        CH2PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2imae(&mut self) -> CH2IMAE_W<5> {
        CH2IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om3(&mut self) -> CH2OM3_W<8> {
        CH2OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_CH2OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch2ocfr](index.html) module"]
pub struct GPTM_CH2OCFR_SPEC;
impl crate::RegisterSpec for GPTM_CH2OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_ch2ocfr::R](R) reader structure"]
impl crate::Readable for GPTM_CH2OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_ch2ocfr::W](W) writer structure"]
impl crate::Writable for GPTM_CH2OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_CH2OCFR to value 0"]
impl crate::Resettable for GPTM_CH2OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
