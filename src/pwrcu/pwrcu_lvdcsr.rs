#[doc = "Register `PWRCU_LVDCSR` reader"]
pub struct R(crate::R<PWRCU_LVDCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_LVDCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_LVDCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_LVDCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_LVDCSR` writer"]
pub struct W(crate::W<PWRCU_LVDCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_LVDCSR_SPEC>;
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
impl From<crate::W<PWRCU_LVDCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_LVDCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODEN` reader - BODEN"]
pub type BODEN_R = crate::BitReader<bool>;
#[doc = "Field `BODEN` writer - BODEN"]
pub type BODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `BODRIS` reader - BODRIS"]
pub type BODRIS_R = crate::BitReader<bool>;
#[doc = "Field `BODRIS` writer - BODRIS"]
pub type BODRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `BODF` reader - BODF"]
pub type BODF_R = crate::BitReader<bool>;
#[doc = "Field `BODF` writer - BODF"]
pub type BODF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `LVDEN` reader - LVDEN"]
pub type LVDEN_R = crate::BitReader<bool>;
#[doc = "Field `LVDEN` writer - LVDEN"]
pub type LVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `LVDS01` reader - LVDS01"]
pub type LVDS01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVDS01` writer - LVDS01"]
pub type LVDS01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRCU_LVDCSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LVDF` reader - LVDF"]
pub type LVDF_R = crate::BitReader<bool>;
#[doc = "Field `LVDF` writer - LVDF"]
pub type LVDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `LVDIWEN` reader - LVDIWEN"]
pub type LVDIWEN_R = crate::BitReader<bool>;
#[doc = "Field `LVDIWEN` writer - LVDIWEN"]
pub type LVDIWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `LVDEWEN` reader - LVDEWEN"]
pub type LVDEWEN_R = crate::BitReader<bool>;
#[doc = "Field `LVDEWEN` writer - LVDEWEN"]
pub type LVDEWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
#[doc = "Field `LVDS2` reader - LVDS2"]
pub type LVDS2_R = crate::BitReader<bool>;
#[doc = "Field `LVDS2` writer - LVDS2"]
pub type LVDS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_LVDCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    pub fn boden(&self) -> BODEN_R {
        BODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    pub fn bodris(&self) -> BODRIS_R {
        BODRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    pub fn bodf(&self) -> BODF_R {
        BODF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    pub fn lvds01(&self) -> LVDS01_R {
        LVDS01_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    pub fn lvdiwen(&self) -> LVDIWEN_R {
        LVDIWEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    pub fn lvdewen(&self) -> LVDEWEN_R {
        LVDEWEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    pub fn lvds2(&self) -> LVDS2_R {
        LVDS2_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    #[must_use]
    pub fn boden(&mut self) -> BODEN_W<0> {
        BODEN_W::new(self)
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    #[must_use]
    pub fn bodris(&mut self) -> BODRIS_W<1> {
        BODRIS_W::new(self)
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    #[must_use]
    pub fn bodf(&mut self) -> BODF_W<3> {
        BODF_W::new(self)
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LVDEN_W<16> {
        LVDEN_W::new(self)
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    #[must_use]
    pub fn lvds01(&mut self) -> LVDS01_W<17> {
        LVDS01_W::new(self)
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    #[must_use]
    pub fn lvdf(&mut self) -> LVDF_W<19> {
        LVDF_W::new(self)
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvdiwen(&mut self) -> LVDIWEN_W<20> {
        LVDIWEN_W::new(self)
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvdewen(&mut self) -> LVDEWEN_W<21> {
        LVDEWEN_W::new(self)
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    #[must_use]
    pub fn lvds2(&mut self) -> LVDS2_W<22> {
        LVDS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_LVDCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_lvdcsr](index.html) module"]
pub struct PWRCU_LVDCSR_SPEC;
impl crate::RegisterSpec for PWRCU_LVDCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_lvdcsr::R](R) reader structure"]
impl crate::Readable for PWRCU_LVDCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_lvdcsr::W](W) writer structure"]
impl crate::Writable for PWRCU_LVDCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_LVDCSR to value 0"]
impl crate::Resettable for PWRCU_LVDCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
