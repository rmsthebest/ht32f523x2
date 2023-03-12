#[doc = "Register `TFR1` reader"]
pub struct R(crate::R<TFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFR1` writer"]
pub struct W(crate::W<TFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFR1_SPEC>;
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
impl From<crate::W<TFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPFF` reader - CMPFF"]
pub type CMPFF_R = crate::BitReader<bool>;
#[doc = "Field `CMPFF` writer - CMPFF"]
pub type CMPFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFR1_SPEC, bool, O>;
#[doc = "Field `CMPRF` reader - CMPRF"]
pub type CMPRF_R = crate::BitReader<bool>;
#[doc = "Field `CMPRF` writer - CMPRF"]
pub type CMPRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFR1_SPEC, bool, O>;
#[doc = "Field `CMPFDEN` reader - CMPFDEN"]
pub type CMPFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPFDEN` writer - CMPFDEN"]
pub type CMPFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFR1_SPEC, bool, O>;
#[doc = "Field `CMPRDEN` reader - CMPRF"]
pub type CMPRDEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPRDEN` writer - CMPRF"]
pub type CMPRDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    pub fn cmpff(&self) -> CMPFF_R {
        CMPFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    pub fn cmprf(&self) -> CMPRF_R {
        CMPRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    pub fn cmpfden(&self) -> CMPFDEN_R {
        CMPFDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMPRF"]
    #[inline(always)]
    pub fn cmprden(&self) -> CMPRDEN_R {
        CMPRDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    #[must_use]
    pub fn cmpff(&mut self) -> CMPFF_W<0> {
        CMPFF_W::new(self)
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    #[must_use]
    pub fn cmprf(&mut self) -> CMPRF_W<1> {
        CMPRF_W::new(self)
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfden(&mut self) -> CMPFDEN_W<8> {
        CMPFDEN_W::new(self)
    }
    #[doc = "Bit 9 - CMPRF"]
    #[inline(always)]
    #[must_use]
    pub fn cmprden(&mut self) -> CMPRDEN_W<9> {
        CMPRDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TFR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr1](index.html) module"]
pub struct TFR1_SPEC;
impl crate::RegisterSpec for TFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfr1::R](R) reader structure"]
impl crate::Readable for TFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfr1::W](W) writer structure"]
impl crate::Writable for TFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFR1 to value 0"]
impl crate::Resettable for TFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
