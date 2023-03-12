#[doc = "Register `WAKUPCR` reader"]
pub struct R(crate::R<WAKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKUPCR` writer"]
pub struct W(crate::W<WAKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKUPCR_SPEC>;
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
impl From<crate::W<WAKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0WEN` reader - EXTI0WEN"]
pub type EXTI0WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0WEN` writer - EXTI0WEN"]
pub type EXTI0WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI1WEN` reader - EXTI1WEN"]
pub type EXTI1WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1WEN` writer - EXTI1WEN"]
pub type EXTI1WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI2WEN` reader - EXTI2WEN"]
pub type EXTI2WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2WEN` writer - EXTI2WEN"]
pub type EXTI2WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI3WEN` reader - EXTI3WEN"]
pub type EXTI3WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3WEN` writer - EXTI3WEN"]
pub type EXTI3WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI4WEN` reader - EXTI4WEN"]
pub type EXTI4WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4WEN` writer - EXTI4WEN"]
pub type EXTI4WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI5WEN` reader - EXTI5WEN"]
pub type EXTI5WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5WEN` writer - EXTI5WEN"]
pub type EXTI5WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI6WEN` reader - EXTI6WEN"]
pub type EXTI6WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6WEN` writer - EXTI6WEN"]
pub type EXTI6WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI7WEN` reader - EXTI7WEN"]
pub type EXTI7WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7WEN` writer - EXTI7WEN"]
pub type EXTI7WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI8WEN` reader - EXTI8WEN"]
pub type EXTI8WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8WEN` writer - EXTI8WEN"]
pub type EXTI8WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI9WEN` reader - EXTI9WEN"]
pub type EXTI9WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9WEN` writer - EXTI9WEN"]
pub type EXTI9WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI10WEN` reader - EXTI10WEN"]
pub type EXTI10WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10WEN` writer - EXTI10WEN"]
pub type EXTI10WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI11WEN` reader - EXTI11WEN"]
pub type EXTI11WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11WEN` writer - EXTI11WEN"]
pub type EXTI11WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI12WEN` reader - EXTI12WEN"]
pub type EXTI12WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12WEN` writer - EXTI12WEN"]
pub type EXTI12WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI13WEN` reader - EXTI13WEN"]
pub type EXTI13WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13WEN` writer - EXTI13WEN"]
pub type EXTI13WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI14WEN` reader - EXTI14WEN"]
pub type EXTI14WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14WEN` writer - EXTI14WEN"]
pub type EXTI14WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EXTI15WEN` reader - EXTI15WEN"]
pub type EXTI15WEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15WEN` writer - EXTI15WEN"]
pub type EXTI15WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
#[doc = "Field `EVWUPIEN` reader - EVWUPIEN"]
pub type EVWUPIEN_R = crate::BitReader<bool>;
#[doc = "Field `EVWUPIEN` writer - EVWUPIEN"]
pub type EVWUPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    pub fn exti0wen(&self) -> EXTI0WEN_R {
        EXTI0WEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    pub fn exti1wen(&self) -> EXTI1WEN_R {
        EXTI1WEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    pub fn exti2wen(&self) -> EXTI2WEN_R {
        EXTI2WEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    pub fn exti3wen(&self) -> EXTI3WEN_R {
        EXTI3WEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    pub fn exti4wen(&self) -> EXTI4WEN_R {
        EXTI4WEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    pub fn exti5wen(&self) -> EXTI5WEN_R {
        EXTI5WEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    pub fn exti6wen(&self) -> EXTI6WEN_R {
        EXTI6WEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    pub fn exti7wen(&self) -> EXTI7WEN_R {
        EXTI7WEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    pub fn exti8wen(&self) -> EXTI8WEN_R {
        EXTI8WEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    pub fn exti9wen(&self) -> EXTI9WEN_R {
        EXTI9WEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    pub fn exti10wen(&self) -> EXTI10WEN_R {
        EXTI10WEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    pub fn exti11wen(&self) -> EXTI11WEN_R {
        EXTI11WEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    pub fn exti12wen(&self) -> EXTI12WEN_R {
        EXTI12WEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    pub fn exti13wen(&self) -> EXTI13WEN_R {
        EXTI13WEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    pub fn exti14wen(&self) -> EXTI14WEN_R {
        EXTI14WEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    pub fn exti15wen(&self) -> EXTI15WEN_R {
        EXTI15WEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    pub fn evwupien(&self) -> EVWUPIEN_R {
        EVWUPIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wen(&mut self) -> EXTI0WEN_W<0> {
        EXTI0WEN_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wen(&mut self) -> EXTI1WEN_W<1> {
        EXTI1WEN_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wen(&mut self) -> EXTI2WEN_W<2> {
        EXTI2WEN_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wen(&mut self) -> EXTI3WEN_W<3> {
        EXTI3WEN_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wen(&mut self) -> EXTI4WEN_W<4> {
        EXTI4WEN_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wen(&mut self) -> EXTI5WEN_W<5> {
        EXTI5WEN_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wen(&mut self) -> EXTI6WEN_W<6> {
        EXTI6WEN_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wen(&mut self) -> EXTI7WEN_W<7> {
        EXTI7WEN_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wen(&mut self) -> EXTI8WEN_W<8> {
        EXTI8WEN_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wen(&mut self) -> EXTI9WEN_W<9> {
        EXTI9WEN_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wen(&mut self) -> EXTI10WEN_W<10> {
        EXTI10WEN_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wen(&mut self) -> EXTI11WEN_W<11> {
        EXTI11WEN_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wen(&mut self) -> EXTI12WEN_W<12> {
        EXTI12WEN_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wen(&mut self) -> EXTI13WEN_W<13> {
        EXTI13WEN_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wen(&mut self) -> EXTI14WEN_W<14> {
        EXTI14WEN_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wen(&mut self) -> EXTI15WEN_W<15> {
        EXTI15WEN_W::new(self)
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn evwupien(&mut self) -> EVWUPIEN_W<31> {
        EVWUPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WAKUPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakupcr](index.html) module"]
pub struct WAKUPCR_SPEC;
impl crate::RegisterSpec for WAKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakupcr::R](R) reader structure"]
impl crate::Readable for WAKUPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakupcr::W](W) writer structure"]
impl crate::Writable for WAKUPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKUPCR to value 0"]
impl crate::Resettable for WAKUPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
