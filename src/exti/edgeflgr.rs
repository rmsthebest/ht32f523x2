#[doc = "Register `EDGEFLGR` reader"]
pub struct R(crate::R<EDGEFLGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDGEFLGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDGEFLGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDGEFLGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDGEFLGR` writer"]
pub struct W(crate::W<EDGEFLGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDGEFLGR_SPEC>;
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
impl From<crate::W<EDGEFLGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDGEFLGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0EDF` reader - EXTI0EDF"]
pub type EXTI0EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0EDF` writer - EXTI0EDF"]
pub type EXTI0EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI1EDF` reader - EXTI1EDF"]
pub type EXTI1EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1EDF` writer - EXTI1EDF"]
pub type EXTI1EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI2EDF` reader - EXTI2EDF"]
pub type EXTI2EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2EDF` writer - EXTI2EDF"]
pub type EXTI2EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI3EDF` reader - EXTI3EDF"]
pub type EXTI3EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3EDF` writer - EXTI3EDF"]
pub type EXTI3EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI4EDF` reader - EXTI4EDF"]
pub type EXTI4EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4EDF` writer - EXTI4EDF"]
pub type EXTI4EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI5EDF` reader - EXTI5EDF"]
pub type EXTI5EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5EDF` writer - EXTI5EDF"]
pub type EXTI5EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI6EDF` reader - EXTI6EDF"]
pub type EXTI6EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6EDF` writer - EXTI6EDF"]
pub type EXTI6EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI7EDF` reader - EXTI7EDF"]
pub type EXTI7EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7EDF` writer - EXTI7EDF"]
pub type EXTI7EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI8EDF` reader - EXTI8EDF"]
pub type EXTI8EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8EDF` writer - EXTI8EDF"]
pub type EXTI8EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI9EDF` reader - EXTI9EDF"]
pub type EXTI9EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9EDF` writer - EXTI9EDF"]
pub type EXTI9EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI10EDF` reader - EXTI10EDF"]
pub type EXTI10EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10EDF` writer - EXTI10EDF"]
pub type EXTI10EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI11EDF` reader - EXTI11EDF"]
pub type EXTI11EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11EDF` writer - EXTI11EDF"]
pub type EXTI11EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI12EDF` reader - EXTI12EDF"]
pub type EXTI12EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12EDF` writer - EXTI12EDF"]
pub type EXTI12EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI13EDF` reader - EXTI13EDF"]
pub type EXTI13EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13EDF` writer - EXTI13EDF"]
pub type EXTI13EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI14EDF` reader - EXTI14EDF"]
pub type EXTI14EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14EDF` writer - EXTI14EDF"]
pub type EXTI14EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
#[doc = "Field `EXTI15EDF` reader - EXTI15EDF"]
pub type EXTI15EDF_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15EDF` writer - EXTI15EDF"]
pub type EXTI15EDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGEFLGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    pub fn exti0edf(&self) -> EXTI0EDF_R {
        EXTI0EDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    pub fn exti1edf(&self) -> EXTI1EDF_R {
        EXTI1EDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    pub fn exti2edf(&self) -> EXTI2EDF_R {
        EXTI2EDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    pub fn exti3edf(&self) -> EXTI3EDF_R {
        EXTI3EDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    pub fn exti4edf(&self) -> EXTI4EDF_R {
        EXTI4EDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    pub fn exti5edf(&self) -> EXTI5EDF_R {
        EXTI5EDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    pub fn exti6edf(&self) -> EXTI6EDF_R {
        EXTI6EDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    pub fn exti7edf(&self) -> EXTI7EDF_R {
        EXTI7EDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    pub fn exti8edf(&self) -> EXTI8EDF_R {
        EXTI8EDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    pub fn exti9edf(&self) -> EXTI9EDF_R {
        EXTI9EDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    pub fn exti10edf(&self) -> EXTI10EDF_R {
        EXTI10EDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    pub fn exti11edf(&self) -> EXTI11EDF_R {
        EXTI11EDF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    pub fn exti12edf(&self) -> EXTI12EDF_R {
        EXTI12EDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    pub fn exti13edf(&self) -> EXTI13EDF_R {
        EXTI13EDF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    pub fn exti14edf(&self) -> EXTI14EDF_R {
        EXTI14EDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    pub fn exti15edf(&self) -> EXTI15EDF_R {
        EXTI15EDF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti0edf(&mut self) -> EXTI0EDF_W<0> {
        EXTI0EDF_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti1edf(&mut self) -> EXTI1EDF_W<1> {
        EXTI1EDF_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti2edf(&mut self) -> EXTI2EDF_W<2> {
        EXTI2EDF_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti3edf(&mut self) -> EXTI3EDF_W<3> {
        EXTI3EDF_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti4edf(&mut self) -> EXTI4EDF_W<4> {
        EXTI4EDF_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti5edf(&mut self) -> EXTI5EDF_W<5> {
        EXTI5EDF_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti6edf(&mut self) -> EXTI6EDF_W<6> {
        EXTI6EDF_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti7edf(&mut self) -> EXTI7EDF_W<7> {
        EXTI7EDF_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti8edf(&mut self) -> EXTI8EDF_W<8> {
        EXTI8EDF_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti9edf(&mut self) -> EXTI9EDF_W<9> {
        EXTI9EDF_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti10edf(&mut self) -> EXTI10EDF_W<10> {
        EXTI10EDF_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti11edf(&mut self) -> EXTI11EDF_W<11> {
        EXTI11EDF_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti12edf(&mut self) -> EXTI12EDF_W<12> {
        EXTI12EDF_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti13edf(&mut self) -> EXTI13EDF_W<13> {
        EXTI13EDF_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti14edf(&mut self) -> EXTI14EDF_W<14> {
        EXTI14EDF_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti15edf(&mut self) -> EXTI15EDF_W<15> {
        EXTI15EDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDGEFLGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edgeflgr](index.html) module"]
pub struct EDGEFLGR_SPEC;
impl crate::RegisterSpec for EDGEFLGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edgeflgr::R](R) reader structure"]
impl crate::Readable for EDGEFLGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edgeflgr::W](W) writer structure"]
impl crate::Writable for EDGEFLGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDGEFLGR to value 0"]
impl crate::Resettable for EDGEFLGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
