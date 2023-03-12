#[doc = "Register `SSCR` reader"]
pub struct R(crate::R<SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCR` writer"]
pub struct W(crate::W<SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCR_SPEC>;
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
impl From<crate::W<SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0SC` reader - EXTI0SC"]
pub type EXTI0SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0SC` writer - EXTI0SC"]
pub type EXTI0SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI1SC` reader - EXTI1SC"]
pub type EXTI1SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1SC` writer - EXTI1SC"]
pub type EXTI1SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI2SC` reader - EXTI2SC"]
pub type EXTI2SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2SC` writer - EXTI2SC"]
pub type EXTI2SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI3SC` reader - EXTI3SC"]
pub type EXTI3SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3SC` writer - EXTI3SC"]
pub type EXTI3SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI4SC` reader - EXTI4SC"]
pub type EXTI4SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4SC` writer - EXTI4SC"]
pub type EXTI4SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI5SC` reader - EXTI5SC"]
pub type EXTI5SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5SC` writer - EXTI5SC"]
pub type EXTI5SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI6SC` reader - EXTI6SC"]
pub type EXTI6SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6SC` writer - EXTI6SC"]
pub type EXTI6SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI7SC` reader - EXTI7SC"]
pub type EXTI7SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7SC` writer - EXTI7SC"]
pub type EXTI7SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI8SC` reader - EXTI8SC"]
pub type EXTI8SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8SC` writer - EXTI8SC"]
pub type EXTI8SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI9SC` reader - EXTI9SC"]
pub type EXTI9SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9SC` writer - EXTI9SC"]
pub type EXTI9SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI10SC` reader - EXTI10SC"]
pub type EXTI10SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10SC` writer - EXTI10SC"]
pub type EXTI10SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI11SC` reader - EXTI11SC"]
pub type EXTI11SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11SC` writer - EXTI11SC"]
pub type EXTI11SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI12SC` reader - EXTI12SC"]
pub type EXTI12SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12SC` writer - EXTI12SC"]
pub type EXTI12SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI13SC` reader - EXTI13SC"]
pub type EXTI13SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13SC` writer - EXTI13SC"]
pub type EXTI13SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI14SC` reader - EXTI14SC"]
pub type EXTI14SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14SC` writer - EXTI14SC"]
pub type EXTI14SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
#[doc = "Field `EXTI15SC` reader - EXTI15SC"]
pub type EXTI15SC_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15SC` writer - EXTI15SC"]
pub type EXTI15SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    pub fn exti0sc(&self) -> EXTI0SC_R {
        EXTI0SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    pub fn exti1sc(&self) -> EXTI1SC_R {
        EXTI1SC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    pub fn exti2sc(&self) -> EXTI2SC_R {
        EXTI2SC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    pub fn exti3sc(&self) -> EXTI3SC_R {
        EXTI3SC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    pub fn exti4sc(&self) -> EXTI4SC_R {
        EXTI4SC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    pub fn exti5sc(&self) -> EXTI5SC_R {
        EXTI5SC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    pub fn exti6sc(&self) -> EXTI6SC_R {
        EXTI6SC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    pub fn exti7sc(&self) -> EXTI7SC_R {
        EXTI7SC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    pub fn exti8sc(&self) -> EXTI8SC_R {
        EXTI8SC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    pub fn exti9sc(&self) -> EXTI9SC_R {
        EXTI9SC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    pub fn exti10sc(&self) -> EXTI10SC_R {
        EXTI10SC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    pub fn exti11sc(&self) -> EXTI11SC_R {
        EXTI11SC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    pub fn exti12sc(&self) -> EXTI12SC_R {
        EXTI12SC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    pub fn exti13sc(&self) -> EXTI13SC_R {
        EXTI13SC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    pub fn exti14sc(&self) -> EXTI14SC_R {
        EXTI14SC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    pub fn exti15sc(&self) -> EXTI15SC_R {
        EXTI15SC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti0sc(&mut self) -> EXTI0SC_W<0> {
        EXTI0SC_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti1sc(&mut self) -> EXTI1SC_W<1> {
        EXTI1SC_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti2sc(&mut self) -> EXTI2SC_W<2> {
        EXTI2SC_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti3sc(&mut self) -> EXTI3SC_W<3> {
        EXTI3SC_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti4sc(&mut self) -> EXTI4SC_W<4> {
        EXTI4SC_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti5sc(&mut self) -> EXTI5SC_W<5> {
        EXTI5SC_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti6sc(&mut self) -> EXTI6SC_W<6> {
        EXTI6SC_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti7sc(&mut self) -> EXTI7SC_W<7> {
        EXTI7SC_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti8sc(&mut self) -> EXTI8SC_W<8> {
        EXTI8SC_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti9sc(&mut self) -> EXTI9SC_W<9> {
        EXTI9SC_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti10sc(&mut self) -> EXTI10SC_W<10> {
        EXTI10SC_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti11sc(&mut self) -> EXTI11SC_W<11> {
        EXTI11SC_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti12sc(&mut self) -> EXTI12SC_W<12> {
        EXTI12SC_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti13sc(&mut self) -> EXTI13SC_W<13> {
        EXTI13SC_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti14sc(&mut self) -> EXTI14SC_W<14> {
        EXTI14SC_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti15sc(&mut self) -> EXTI15SC_W<15> {
        EXTI15SC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscr](index.html) module"]
pub struct SSCR_SPEC;
impl crate::RegisterSpec for SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sscr::R](R) reader structure"]
impl crate::Readable for SSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sscr::W](W) writer structure"]
impl crate::Writable for SSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSCR to value 0"]
impl crate::Resettable for SSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
