#[doc = "Register `WAKUPFLG` reader"]
pub struct R(crate::R<WAKUPFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKUPFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKUPFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKUPFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKUPFLG` writer"]
pub struct W(crate::W<WAKUPFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKUPFLG_SPEC>;
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
impl From<crate::W<WAKUPFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKUPFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0WFL` reader - EXTI0WFL"]
pub type EXTI0WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0WFL` writer - EXTI0WFL"]
pub type EXTI0WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI1WFL` reader - EXTI1WFL"]
pub type EXTI1WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1WFL` writer - EXTI1WFL"]
pub type EXTI1WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI2WFL` reader - EXTI2WFL"]
pub type EXTI2WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2WFL` writer - EXTI2WFL"]
pub type EXTI2WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI3WFL` reader - EXTI3WFL"]
pub type EXTI3WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3WFL` writer - EXTI3WFL"]
pub type EXTI3WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI4WFL` reader - EXTI4WFL"]
pub type EXTI4WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4WFL` writer - EXTI4WFL"]
pub type EXTI4WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI5WFL` reader - EXTI5WFL"]
pub type EXTI5WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5WFL` writer - EXTI5WFL"]
pub type EXTI5WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI6WFL` reader - EXTI6WFL"]
pub type EXTI6WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6WFL` writer - EXTI6WFL"]
pub type EXTI6WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI7WFL` reader - EXTI7WFL"]
pub type EXTI7WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7WFL` writer - EXTI7WFL"]
pub type EXTI7WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI8WFL` reader - EXTI8WFL"]
pub type EXTI8WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8WFL` writer - EXTI8WFL"]
pub type EXTI8WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI9WFL` reader - EXTI9WFL"]
pub type EXTI9WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9WFL` writer - EXTI9WFL"]
pub type EXTI9WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI10WFL` reader - EXTI10WFL"]
pub type EXTI10WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10WFL` writer - EXTI10WFL"]
pub type EXTI10WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI11WFL` reader - EXTI11WFL"]
pub type EXTI11WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11WFL` writer - EXTI11WFL"]
pub type EXTI11WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI12WFL` reader - EXTI12WFL"]
pub type EXTI12WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12WFL` writer - EXTI12WFL"]
pub type EXTI12WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI13WFL` reader - EXTI13WFL"]
pub type EXTI13WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13WFL` writer - EXTI13WFL"]
pub type EXTI13WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI14WFL` reader - EXTI14WFL"]
pub type EXTI14WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14WFL` writer - EXTI14WFL"]
pub type EXTI14WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
#[doc = "Field `EXTI15WFL` reader - EXTI15WFL"]
pub type EXTI15WFL_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15WFL` writer - EXTI15WFL"]
pub type EXTI15WFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKUPFLG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    pub fn exti0wfl(&self) -> EXTI0WFL_R {
        EXTI0WFL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    pub fn exti1wfl(&self) -> EXTI1WFL_R {
        EXTI1WFL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    pub fn exti2wfl(&self) -> EXTI2WFL_R {
        EXTI2WFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    pub fn exti3wfl(&self) -> EXTI3WFL_R {
        EXTI3WFL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    pub fn exti4wfl(&self) -> EXTI4WFL_R {
        EXTI4WFL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    pub fn exti5wfl(&self) -> EXTI5WFL_R {
        EXTI5WFL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    pub fn exti6wfl(&self) -> EXTI6WFL_R {
        EXTI6WFL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    pub fn exti7wfl(&self) -> EXTI7WFL_R {
        EXTI7WFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    pub fn exti8wfl(&self) -> EXTI8WFL_R {
        EXTI8WFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    pub fn exti9wfl(&self) -> EXTI9WFL_R {
        EXTI9WFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    pub fn exti10wfl(&self) -> EXTI10WFL_R {
        EXTI10WFL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    pub fn exti11wfl(&self) -> EXTI11WFL_R {
        EXTI11WFL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    pub fn exti12wfl(&self) -> EXTI12WFL_R {
        EXTI12WFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    pub fn exti13wfl(&self) -> EXTI13WFL_R {
        EXTI13WFL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    pub fn exti14wfl(&self) -> EXTI14WFL_R {
        EXTI14WFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    pub fn exti15wfl(&self) -> EXTI15WFL_R {
        EXTI15WFL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wfl(&mut self) -> EXTI0WFL_W<0> {
        EXTI0WFL_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wfl(&mut self) -> EXTI1WFL_W<1> {
        EXTI1WFL_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wfl(&mut self) -> EXTI2WFL_W<2> {
        EXTI2WFL_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wfl(&mut self) -> EXTI3WFL_W<3> {
        EXTI3WFL_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wfl(&mut self) -> EXTI4WFL_W<4> {
        EXTI4WFL_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wfl(&mut self) -> EXTI5WFL_W<5> {
        EXTI5WFL_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wfl(&mut self) -> EXTI6WFL_W<6> {
        EXTI6WFL_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wfl(&mut self) -> EXTI7WFL_W<7> {
        EXTI7WFL_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wfl(&mut self) -> EXTI8WFL_W<8> {
        EXTI8WFL_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wfl(&mut self) -> EXTI9WFL_W<9> {
        EXTI9WFL_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wfl(&mut self) -> EXTI10WFL_W<10> {
        EXTI10WFL_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wfl(&mut self) -> EXTI11WFL_W<11> {
        EXTI11WFL_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wfl(&mut self) -> EXTI12WFL_W<12> {
        EXTI12WFL_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wfl(&mut self) -> EXTI13WFL_W<13> {
        EXTI13WFL_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wfl(&mut self) -> EXTI14WFL_W<14> {
        EXTI14WFL_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wfl(&mut self) -> EXTI15WFL_W<15> {
        EXTI15WFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WAKUPFLG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakupflg](index.html) module"]
pub struct WAKUPFLG_SPEC;
impl crate::RegisterSpec for WAKUPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakupflg::R](R) reader structure"]
impl crate::Readable for WAKUPFLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakupflg::W](W) writer structure"]
impl crate::Writable for WAKUPFLG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKUPFLG to value 0"]
impl crate::Resettable for WAKUPFLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
