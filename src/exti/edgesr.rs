#[doc = "Register `EDGESR` reader"]
pub struct R(crate::R<EDGESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDGESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDGESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDGESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDGESR` writer"]
pub struct W(crate::W<EDGESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDGESR_SPEC>;
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
impl From<crate::W<EDGESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDGESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0EDS` reader - EXTI0EDS"]
pub type EXTI0EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0EDS` writer - EXTI0EDS"]
pub type EXTI0EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI1EDS` reader - EXTI1EDS"]
pub type EXTI1EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1EDS` writer - EXTI1EDS"]
pub type EXTI1EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI2EDS` reader - EXTI2EDS"]
pub type EXTI2EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2EDS` writer - EXTI2EDS"]
pub type EXTI2EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI3EDS` reader - EXTI3EDS"]
pub type EXTI3EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3EDS` writer - EXTI3EDS"]
pub type EXTI3EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI4EDS` reader - EXTI4EDS"]
pub type EXTI4EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4EDS` writer - EXTI4EDS"]
pub type EXTI4EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI5EDS` reader - EXTI5EDS"]
pub type EXTI5EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5EDS` writer - EXTI5EDS"]
pub type EXTI5EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI6EDS` reader - EXTI6EDS"]
pub type EXTI6EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6EDS` writer - EXTI6EDS"]
pub type EXTI6EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI7EDS` reader - EXTI7EDS"]
pub type EXTI7EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7EDS` writer - EXTI7EDS"]
pub type EXTI7EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI8EDS` reader - EXTI8EDS"]
pub type EXTI8EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8EDS` writer - EXTI8EDS"]
pub type EXTI8EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI9EDS` reader - EXTI9EDS"]
pub type EXTI9EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9EDS` writer - EXTI9EDS"]
pub type EXTI9EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI10EDS` reader - EXTI10EDS"]
pub type EXTI10EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10EDS` writer - EXTI10EDS"]
pub type EXTI10EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI11EDS` reader - EXTI11EDS"]
pub type EXTI11EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11EDS` writer - EXTI11EDS"]
pub type EXTI11EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI12EDS` reader - EXTI12EDS"]
pub type EXTI12EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12EDS` writer - EXTI12EDS"]
pub type EXTI12EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI13EDS` reader - EXTI13EDS"]
pub type EXTI13EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13EDS` writer - EXTI13EDS"]
pub type EXTI13EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI14EDS` reader - EXTI14EDS"]
pub type EXTI14EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14EDS` writer - EXTI14EDS"]
pub type EXTI14EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
#[doc = "Field `EXTI15EDS` reader - EXTI15EDS"]
pub type EXTI15EDS_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15EDS` writer - EXTI15EDS"]
pub type EXTI15EDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGESR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    pub fn exti0eds(&self) -> EXTI0EDS_R {
        EXTI0EDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    pub fn exti1eds(&self) -> EXTI1EDS_R {
        EXTI1EDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    pub fn exti2eds(&self) -> EXTI2EDS_R {
        EXTI2EDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    pub fn exti3eds(&self) -> EXTI3EDS_R {
        EXTI3EDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    pub fn exti4eds(&self) -> EXTI4EDS_R {
        EXTI4EDS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    pub fn exti5eds(&self) -> EXTI5EDS_R {
        EXTI5EDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    pub fn exti6eds(&self) -> EXTI6EDS_R {
        EXTI6EDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    pub fn exti7eds(&self) -> EXTI7EDS_R {
        EXTI7EDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    pub fn exti8eds(&self) -> EXTI8EDS_R {
        EXTI8EDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    pub fn exti9eds(&self) -> EXTI9EDS_R {
        EXTI9EDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    pub fn exti10eds(&self) -> EXTI10EDS_R {
        EXTI10EDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    pub fn exti11eds(&self) -> EXTI11EDS_R {
        EXTI11EDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    pub fn exti12eds(&self) -> EXTI12EDS_R {
        EXTI12EDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    pub fn exti13eds(&self) -> EXTI13EDS_R {
        EXTI13EDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    pub fn exti14eds(&self) -> EXTI14EDS_R {
        EXTI14EDS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    pub fn exti15eds(&self) -> EXTI15EDS_R {
        EXTI15EDS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti0eds(&mut self) -> EXTI0EDS_W<0> {
        EXTI0EDS_W::new(self)
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti1eds(&mut self) -> EXTI1EDS_W<1> {
        EXTI1EDS_W::new(self)
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti2eds(&mut self) -> EXTI2EDS_W<2> {
        EXTI2EDS_W::new(self)
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti3eds(&mut self) -> EXTI3EDS_W<3> {
        EXTI3EDS_W::new(self)
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti4eds(&mut self) -> EXTI4EDS_W<4> {
        EXTI4EDS_W::new(self)
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti5eds(&mut self) -> EXTI5EDS_W<5> {
        EXTI5EDS_W::new(self)
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti6eds(&mut self) -> EXTI6EDS_W<6> {
        EXTI6EDS_W::new(self)
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti7eds(&mut self) -> EXTI7EDS_W<7> {
        EXTI7EDS_W::new(self)
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti8eds(&mut self) -> EXTI8EDS_W<8> {
        EXTI8EDS_W::new(self)
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti9eds(&mut self) -> EXTI9EDS_W<9> {
        EXTI9EDS_W::new(self)
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti10eds(&mut self) -> EXTI10EDS_W<10> {
        EXTI10EDS_W::new(self)
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti11eds(&mut self) -> EXTI11EDS_W<11> {
        EXTI11EDS_W::new(self)
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti12eds(&mut self) -> EXTI12EDS_W<12> {
        EXTI12EDS_W::new(self)
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti13eds(&mut self) -> EXTI13EDS_W<13> {
        EXTI13EDS_W::new(self)
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti14eds(&mut self) -> EXTI14EDS_W<14> {
        EXTI14EDS_W::new(self)
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti15eds(&mut self) -> EXTI15EDS_W<15> {
        EXTI15EDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDGESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edgesr](index.html) module"]
pub struct EDGESR_SPEC;
impl crate::RegisterSpec for EDGESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edgesr::R](R) reader structure"]
impl crate::Readable for EDGESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edgesr::W](W) writer structure"]
impl crate::Writable for EDGESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDGESR to value 0"]
impl crate::Resettable for EDGESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
