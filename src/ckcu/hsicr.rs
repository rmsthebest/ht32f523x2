#[doc = "Register `HSICR` reader"]
pub struct R(crate::R<HSICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSICR` writer"]
pub struct W(crate::W<HSICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSICR_SPEC>;
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
impl From<crate::W<HSICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMEN` reader - TRIMEN"]
pub type TRIMEN_R = crate::BitReader<bool>;
#[doc = "Field `TRIMEN` writer - TRIMEN"]
pub type TRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSICR_SPEC, bool, O>;
#[doc = "Field `ATCEN` reader - ATCEN"]
pub type ATCEN_R = crate::BitReader<bool>;
#[doc = "Field `ATCEN` writer - ATCEN"]
pub type ATCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSICR_SPEC, bool, O>;
#[doc = "Field `TMSEL` reader - TMSEL"]
pub type TMSEL_R = crate::BitReader<bool>;
#[doc = "Field `TMSEL` writer - TMSEL"]
pub type TMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSICR_SPEC, bool, O>;
#[doc = "Field `REFCLKSEL` reader - REFCLKSEL"]
pub type REFCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `REFCLKSEL` writer - REFCLKSEL"]
pub type REFCLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSICR_SPEC, bool, O>;
#[doc = "Field `FLOCK` reader - FLOCK"]
pub type FLOCK_R = crate::BitReader<bool>;
#[doc = "Field `FLOCK` writer - FLOCK"]
pub type FLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSICR_SPEC, bool, O>;
#[doc = "Field `HSIFINE` reader - HSIFINE"]
pub type HSIFINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSIFINE` writer - HSIFINE"]
pub type HSIFINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSICR_SPEC, u8, u8, 8, O>;
#[doc = "Field `HSICOARSE` reader - HSICOARSE"]
pub type HSICOARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSICOARSE` writer - HSICOARSE"]
pub type HSICOARSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSICR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    pub fn trimen(&self) -> TRIMEN_R {
        TRIMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    pub fn atcen(&self) -> ATCEN_R {
        ATCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    pub fn tmsel(&self) -> TMSEL_R {
        TMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    pub fn refclksel(&self) -> REFCLKSEL_R {
        REFCLKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    pub fn flock(&self) -> FLOCK_R {
        FLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    pub fn hsifine(&self) -> HSIFINE_R {
        HSIFINE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    pub fn hsicoarse(&self) -> HSICOARSE_R {
        HSICOARSE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    #[must_use]
    pub fn trimen(&mut self) -> TRIMEN_W<0> {
        TRIMEN_W::new(self)
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    #[must_use]
    pub fn atcen(&mut self) -> ATCEN_W<1> {
        ATCEN_W::new(self)
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn tmsel(&mut self) -> TMSEL_W<4> {
        TMSEL_W::new(self)
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn refclksel(&mut self) -> REFCLKSEL_W<5> {
        REFCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn flock(&mut self) -> FLOCK_W<7> {
        FLOCK_W::new(self)
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    #[must_use]
    pub fn hsifine(&mut self) -> HSIFINE_W<16> {
        HSIFINE_W::new(self)
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    #[must_use]
    pub fn hsicoarse(&mut self) -> HSICOARSE_W<24> {
        HSICOARSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsicr](index.html) module"]
pub struct HSICR_SPEC;
impl crate::RegisterSpec for HSICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsicr::R](R) reader structure"]
impl crate::Readable for HSICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsicr::W](W) writer structure"]
impl crate::Writable for HSICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSICR to value 0"]
impl crate::Resettable for HSICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
