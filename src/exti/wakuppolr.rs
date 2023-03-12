#[doc = "Register `WAKUPPOLR` reader"]
pub struct R(crate::R<WAKUPPOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKUPPOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKUPPOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKUPPOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKUPPOLR` writer"]
pub struct W(crate::W<WAKUPPOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKUPPOLR_SPEC>;
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
impl From<crate::W<WAKUPPOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKUPPOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0WPOL` reader - EXTI0WPOL"]
pub type EXTI0WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0WPOL` writer - EXTI0WPOL"]
pub type EXTI0WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI1WPOL` reader - EXTI1WPOL"]
pub type EXTI1WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1WPOL` writer - EXTI1WPOL"]
pub type EXTI1WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI2WPOL` reader - EXTI2WPOL"]
pub type EXTI2WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2WPOL` writer - EXTI2WPOL"]
pub type EXTI2WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI3WPOL` reader - EXTI3WPOL"]
pub type EXTI3WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3WPOL` writer - EXTI3WPOL"]
pub type EXTI3WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI4WPOL` reader - EXTI4WPOL"]
pub type EXTI4WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4WPOL` writer - EXTI4WPOL"]
pub type EXTI4WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI5WPOL` reader - EXTI5WPOL"]
pub type EXTI5WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5WPOL` writer - EXTI5WPOL"]
pub type EXTI5WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI6WPOL` reader - EXTI6WPOL"]
pub type EXTI6WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6WPOL` writer - EXTI6WPOL"]
pub type EXTI6WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI7WPOL` reader - EXTI7WPOL"]
pub type EXTI7WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7WPOL` writer - EXTI7WPOL"]
pub type EXTI7WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI8WPOL` reader - EXTI8WPOL"]
pub type EXTI8WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8WPOL` writer - EXTI8WPOL"]
pub type EXTI8WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI9WPOL` reader - EXTI9WPOL"]
pub type EXTI9WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9WPOL` writer - EXTI9WPOL"]
pub type EXTI9WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI10WPOL` reader - EXTI10WPOL"]
pub type EXTI10WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10WPOL` writer - EXTI10WPOL"]
pub type EXTI10WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI11WPOL` reader - EXTI11WPOL"]
pub type EXTI11WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11WPOL` writer - EXTI11WPOL"]
pub type EXTI11WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI12WPOL` reader - EXTI12WPOL"]
pub type EXTI12WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12WPOL` writer - EXTI12WPOL"]
pub type EXTI12WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI13WPOL` reader - EXTI13WPOL"]
pub type EXTI13WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13WPOL` writer - EXTI13WPOL"]
pub type EXTI13WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI14WPOL` reader - EXTI14WPOL"]
pub type EXTI14WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14WPOL` writer - EXTI14WPOL"]
pub type EXTI14WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
#[doc = "Field `EXTI15WPOL` reader - EXTI15WPOL"]
pub type EXTI15WPOL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15WPOL` writer - EXTI15WPOL"]
pub type EXTI15WPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPPOLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    pub fn exti0wpol(&self) -> EXTI0WPOL_R {
        EXTI0WPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    pub fn exti1wpol(&self) -> EXTI1WPOL_R {
        EXTI1WPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    pub fn exti2wpol(&self) -> EXTI2WPOL_R {
        EXTI2WPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    pub fn exti3wpol(&self) -> EXTI3WPOL_R {
        EXTI3WPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    pub fn exti4wpol(&self) -> EXTI4WPOL_R {
        EXTI4WPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    pub fn exti5wpol(&self) -> EXTI5WPOL_R {
        EXTI5WPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    pub fn exti6wpol(&self) -> EXTI6WPOL_R {
        EXTI6WPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    pub fn exti7wpol(&self) -> EXTI7WPOL_R {
        EXTI7WPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    pub fn exti8wpol(&self) -> EXTI8WPOL_R {
        EXTI8WPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    pub fn exti9wpol(&self) -> EXTI9WPOL_R {
        EXTI9WPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    pub fn exti10wpol(&self) -> EXTI10WPOL_R {
        EXTI10WPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    pub fn exti11wpol(&self) -> EXTI11WPOL_R {
        EXTI11WPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    pub fn exti12wpol(&self) -> EXTI12WPOL_R {
        EXTI12WPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    pub fn exti13wpol(&self) -> EXTI13WPOL_R {
        EXTI13WPOL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    pub fn exti14wpol(&self) -> EXTI14WPOL_R {
        EXTI14WPOL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    pub fn exti15wpol(&self) -> EXTI15WPOL_R {
        EXTI15WPOL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wpol(&mut self) -> EXTI0WPOL_W<0> {
        EXTI0WPOL_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wpol(&mut self) -> EXTI1WPOL_W<1> {
        EXTI1WPOL_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wpol(&mut self) -> EXTI2WPOL_W<2> {
        EXTI2WPOL_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wpol(&mut self) -> EXTI3WPOL_W<3> {
        EXTI3WPOL_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wpol(&mut self) -> EXTI4WPOL_W<4> {
        EXTI4WPOL_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wpol(&mut self) -> EXTI5WPOL_W<5> {
        EXTI5WPOL_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wpol(&mut self) -> EXTI6WPOL_W<6> {
        EXTI6WPOL_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wpol(&mut self) -> EXTI7WPOL_W<7> {
        EXTI7WPOL_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wpol(&mut self) -> EXTI8WPOL_W<8> {
        EXTI8WPOL_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wpol(&mut self) -> EXTI9WPOL_W<9> {
        EXTI9WPOL_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wpol(&mut self) -> EXTI10WPOL_W<10> {
        EXTI10WPOL_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wpol(&mut self) -> EXTI11WPOL_W<11> {
        EXTI11WPOL_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wpol(&mut self) -> EXTI12WPOL_W<12> {
        EXTI12WPOL_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wpol(&mut self) -> EXTI13WPOL_W<13> {
        EXTI13WPOL_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wpol(&mut self) -> EXTI14WPOL_W<14> {
        EXTI14WPOL_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wpol(&mut self) -> EXTI15WPOL_W<15> {
        EXTI15WPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WAKUPPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakuppolr](index.html) module"]
pub struct WAKUPPOLR_SPEC;
impl crate::RegisterSpec for WAKUPPOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakuppolr::R](R) reader structure"]
impl crate::Readable for WAKUPPOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakuppolr::W](W) writer structure"]
impl crate::Writable for WAKUPPOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKUPPOLR to value 0"]
impl crate::Resettable for WAKUPPOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
