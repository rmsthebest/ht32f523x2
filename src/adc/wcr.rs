#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADWLE` reader - ADWLE"]
pub type ADWLE_R = crate::BitReader<bool>;
#[doc = "Field `ADWLE` writer - ADWLE"]
pub type ADWLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `ADWUE` reader - ADWUE"]
pub type ADWUE_R = crate::BitReader<bool>;
#[doc = "Field `ADWUE` writer - ADWUE"]
pub type ADWUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `ADWALL` reader - ADWALL"]
pub type ADWALL_R = crate::BitReader<bool>;
#[doc = "Field `ADWALL` writer - ADWALL"]
pub type ADWALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `ADWCH` reader - ADWCH"]
pub type ADWCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADWCH` writer - ADWCH"]
pub type ADWCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADLCH` reader - ADLCH"]
pub type ADLCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADLCH` writer - ADLCH"]
pub type ADLCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADUCH` reader - ADUCH"]
pub type ADUCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADUCH` writer - ADUCH"]
pub type ADUCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    pub fn adwle(&self) -> ADWLE_R {
        ADWLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    pub fn adwue(&self) -> ADWUE_R {
        ADWUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    pub fn adwall(&self) -> ADWALL_R {
        ADWALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    pub fn adwch(&self) -> ADWCH_R {
        ADWCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    pub fn adlch(&self) -> ADLCH_R {
        ADLCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    pub fn aduch(&self) -> ADUCH_R {
        ADUCH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    #[must_use]
    pub fn adwle(&mut self) -> ADWLE_W<0> {
        ADWLE_W::new(self)
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    #[must_use]
    pub fn adwue(&mut self) -> ADWUE_W<1> {
        ADWUE_W::new(self)
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    #[must_use]
    pub fn adwall(&mut self) -> ADWALL_W<2> {
        ADWALL_W::new(self)
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    #[must_use]
    pub fn adwch(&mut self) -> ADWCH_W<8> {
        ADWCH_W::new(self)
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    #[must_use]
    pub fn adlch(&mut self) -> ADLCH_W<16> {
        ADLCH_W::new(self)
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    #[must_use]
    pub fn aduch(&mut self) -> ADUCH_W<24> {
        ADUCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
