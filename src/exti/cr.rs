#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0EN` reader - EXTI0EN"]
pub type EXTI0EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0EN` writer - EXTI0EN"]
pub type EXTI0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI1EN` reader - EXTI1EN"]
pub type EXTI1EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1EN` writer - EXTI1EN"]
pub type EXTI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI2EN` reader - EXTI2EN"]
pub type EXTI2EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2EN` writer - EXTI2EN"]
pub type EXTI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI3EN` reader - EXTI3EN"]
pub type EXTI3EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3EN` writer - EXTI3EN"]
pub type EXTI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI4EN` reader - EXTI4EN"]
pub type EXTI4EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4EN` writer - EXTI4EN"]
pub type EXTI4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI5EN` reader - EXTI5EN"]
pub type EXTI5EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5EN` writer - EXTI5EN"]
pub type EXTI5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI6EN` reader - EXTI6EN"]
pub type EXTI6EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6EN` writer - EXTI6EN"]
pub type EXTI6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI7EN` reader - EXTI7EN"]
pub type EXTI7EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7EN` writer - EXTI7EN"]
pub type EXTI7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI8EN` reader - EXTI8EN"]
pub type EXTI8EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8EN` writer - EXTI8EN"]
pub type EXTI8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI9EN` reader - EXTI9EN"]
pub type EXTI9EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9EN` writer - EXTI9EN"]
pub type EXTI9EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI10EN` reader - EXTI10EN"]
pub type EXTI10EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10EN` writer - EXTI10EN"]
pub type EXTI10EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI11EN` reader - EXTI11EN"]
pub type EXTI11EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11EN` writer - EXTI11EN"]
pub type EXTI11EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI12EN` reader - EXTI12EN"]
pub type EXTI12EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12EN` writer - EXTI12EN"]
pub type EXTI12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI13EN` reader - EXTI13EN"]
pub type EXTI13EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13EN` writer - EXTI13EN"]
pub type EXTI13EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI14EN` reader - EXTI14EN"]
pub type EXTI14EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14EN` writer - EXTI14EN"]
pub type EXTI14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTI15EN` reader - EXTI15EN"]
pub type EXTI15EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15EN` writer - EXTI15EN"]
pub type EXTI15EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    pub fn exti0en(&self) -> EXTI0EN_R {
        EXTI0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    pub fn exti1en(&self) -> EXTI1EN_R {
        EXTI1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    pub fn exti2en(&self) -> EXTI2EN_R {
        EXTI2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    pub fn exti3en(&self) -> EXTI3EN_R {
        EXTI3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    pub fn exti4en(&self) -> EXTI4EN_R {
        EXTI4EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    pub fn exti5en(&self) -> EXTI5EN_R {
        EXTI5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    pub fn exti6en(&self) -> EXTI6EN_R {
        EXTI6EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    pub fn exti7en(&self) -> EXTI7EN_R {
        EXTI7EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    pub fn exti8en(&self) -> EXTI8EN_R {
        EXTI8EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    pub fn exti9en(&self) -> EXTI9EN_R {
        EXTI9EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    pub fn exti10en(&self) -> EXTI10EN_R {
        EXTI10EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    pub fn exti11en(&self) -> EXTI11EN_R {
        EXTI11EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    pub fn exti12en(&self) -> EXTI12EN_R {
        EXTI12EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    pub fn exti13en(&self) -> EXTI13EN_R {
        EXTI13EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    pub fn exti14en(&self) -> EXTI14EN_R {
        EXTI14EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    pub fn exti15en(&self) -> EXTI15EN_R {
        EXTI15EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0en(&mut self) -> EXTI0EN_W<0> {
        EXTI0EN_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1en(&mut self) -> EXTI1EN_W<1> {
        EXTI1EN_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2en(&mut self) -> EXTI2EN_W<2> {
        EXTI2EN_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3en(&mut self) -> EXTI3EN_W<3> {
        EXTI3EN_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4en(&mut self) -> EXTI4EN_W<4> {
        EXTI4EN_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5en(&mut self) -> EXTI5EN_W<5> {
        EXTI5EN_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6en(&mut self) -> EXTI6EN_W<6> {
        EXTI6EN_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7en(&mut self) -> EXTI7EN_W<7> {
        EXTI7EN_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8en(&mut self) -> EXTI8EN_W<8> {
        EXTI8EN_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9en(&mut self) -> EXTI9EN_W<9> {
        EXTI9EN_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10en(&mut self) -> EXTI10EN_W<10> {
        EXTI10EN_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11en(&mut self) -> EXTI11EN_W<11> {
        EXTI11EN_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12en(&mut self) -> EXTI12EN_W<12> {
        EXTI12EN_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13en(&mut self) -> EXTI13EN_W<13> {
        EXTI13EN_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14en(&mut self) -> EXTI14EN_W<14> {
        EXTI14EN_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15en(&mut self) -> EXTI15EN_W<15> {
        EXTI15EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
