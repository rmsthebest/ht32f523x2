#[doc = "Register `GCSR` reader"]
pub struct R(crate::R<GCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCSR` writer"]
pub struct W(crate::W<GCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCSR_SPEC>;
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
impl From<crate::W<GCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLRDY` reader - PLLRDY"]
pub type PLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDY` writer - PLLRDY"]
pub type PLLRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSR_SPEC, bool, O>;
#[doc = "Field `HSERDY` reader - HSERDY"]
pub type HSERDY_R = crate::BitReader<bool>;
#[doc = "Field `HSERDY` writer - HSERDY"]
pub type HSERDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSR_SPEC, bool, O>;
#[doc = "Field `HSIRDY` reader - HSIRDY"]
pub type HSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDY` writer - HSIRDY"]
pub type HSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSR_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - LSERDY"]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSERDY` writer - LSERDY"]
pub type LSERDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - LSIRDY"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDY` writer - LSIRDY"]
pub type LSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdy(&mut self) -> PLLRDY_W<1> {
        PLLRDY_W::new(self)
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<2> {
        HSERDY_W::new(self)
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<3> {
        HSIRDY_W::new(self)
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<4> {
        LSERDY_W::new(self)
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<5> {
        LSIRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcsr](index.html) module"]
pub struct GCSR_SPEC;
impl crate::RegisterSpec for GCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcsr::R](R) reader structure"]
impl crate::Readable for GCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcsr::W](W) writer structure"]
impl crate::Writable for GCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCSR to value 0"]
impl crate::Resettable for GCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
