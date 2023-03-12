#[doc = "Register `GCIR` reader"]
pub struct R(crate::R<GCIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCIR` writer"]
pub struct W(crate::W<GCIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCIR_SPEC>;
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
impl From<crate::W<GCIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSF` reader - CKSF"]
pub type CKSF_R = crate::BitReader<bool>;
#[doc = "Field `CKSF` writer - CKSF"]
pub type CKSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYF` reader - PLLRDYF"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` writer - PLLRDYF"]
pub type PLLRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `HSERDYF` reader - HSERDYF"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` writer - HSERDYF"]
pub type HSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYF` reader - HSIRDYF"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` writer - HSIRDYF"]
pub type HSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `LSERDYF` reader - LSERDYF"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` writer - LSERDYF"]
pub type LSERDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYF` reader - LSIRDYF"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYF` writer - LSIRDYF"]
pub type LSIRDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `CKSIE` reader - CKSIE"]
pub type CKSIE_R = crate::BitReader<bool>;
#[doc = "Field `CKSIE` writer - CKSIE"]
pub type CKSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYIE` reader - PLLRDYIE"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - PLLRDYIE"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - HSERDYIE"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSERDYIE"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - HSIRDYIE"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSIRDYIE"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `LSERDYIE` reader - LSERDYIE"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - LSERDYIE"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYIE` reader - LSIRDYIE"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSIRDYIE"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    pub fn cksf(&self) -> CKSF_R {
        CKSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    pub fn cksie(&self) -> CKSIE_R {
        CKSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    #[must_use]
    pub fn cksf(&mut self) -> CKSF_W<0> {
        CKSF_W::new(self)
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyf(&mut self) -> PLLRDYF_W<2> {
        PLLRDYF_W::new(self)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<3> {
        HSERDYF_W::new(self)
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<4> {
        HSIRDYF_W::new(self)
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<5> {
        LSERDYF_W::new(self)
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<6> {
        LSIRDYF_W::new(self)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    #[must_use]
    pub fn cksie(&mut self) -> CKSIE_W<16> {
        CKSIE_W::new(self)
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<18> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<19> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<20> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<21> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<22> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcir](index.html) module"]
pub struct GCIR_SPEC;
impl crate::RegisterSpec for GCIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcir::R](R) reader structure"]
impl crate::Readable for GCIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcir::W](W) writer structure"]
impl crate::Writable for GCIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCIR to value 0"]
impl crate::Resettable for GCIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
