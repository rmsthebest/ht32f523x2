#[doc = "Register `ESSR1` reader"]
pub struct R(crate::R<ESSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESSR1` writer"]
pub struct W(crate::W<ESSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESSR1_SPEC>;
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
impl From<crate::W<ESSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8PIN` reader - EXTI8PIN"]
pub type EXTI8PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI8PIN` writer - EXTI8PIN"]
pub type EXTI8PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI9PIN` reader - EXTI9PIN"]
pub type EXTI9PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI9PIN` writer - EXTI9PIN"]
pub type EXTI9PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI10PIN` reader - EXTI10PIN"]
pub type EXTI10PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI10PIN` writer - EXTI10PIN"]
pub type EXTI10PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI11PIN` reader - EXTI11PIN"]
pub type EXTI11PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI11PIN` writer - EXTI11PIN"]
pub type EXTI11PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI12PIN` reader - EXTI12PIN"]
pub type EXTI12PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI12PIN` writer - EXTI12PIN"]
pub type EXTI12PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI13PIN` reader - EXTI13PIN"]
pub type EXTI13PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI13PIN` writer - EXTI13PIN"]
pub type EXTI13PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI14PIN` reader - EXTI14PIN"]
pub type EXTI14PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI14PIN` writer - EXTI14PIN"]
pub type EXTI14PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI15PIN` reader - EXTI15PIN"]
pub type EXTI15PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI15PIN` writer - EXTI15PIN"]
pub type EXTI15PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    pub fn exti8pin(&self) -> EXTI8PIN_R {
        EXTI8PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    pub fn exti9pin(&self) -> EXTI9PIN_R {
        EXTI9PIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    pub fn exti10pin(&self) -> EXTI10PIN_R {
        EXTI10PIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    pub fn exti11pin(&self) -> EXTI11PIN_R {
        EXTI11PIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    pub fn exti12pin(&self) -> EXTI12PIN_R {
        EXTI12PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    pub fn exti13pin(&self) -> EXTI13PIN_R {
        EXTI13PIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    pub fn exti14pin(&self) -> EXTI14PIN_R {
        EXTI14PIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    pub fn exti15pin(&self) -> EXTI15PIN_R {
        EXTI15PIN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8pin(&mut self) -> EXTI8PIN_W<0> {
        EXTI8PIN_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9pin(&mut self) -> EXTI9PIN_W<4> {
        EXTI9PIN_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10pin(&mut self) -> EXTI10PIN_W<8> {
        EXTI10PIN_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11pin(&mut self) -> EXTI11PIN_W<12> {
        EXTI11PIN_W::new(self)
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12pin(&mut self) -> EXTI12PIN_W<16> {
        EXTI12PIN_W::new(self)
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13pin(&mut self) -> EXTI13PIN_W<20> {
        EXTI13PIN_W::new(self)
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14pin(&mut self) -> EXTI14PIN_W<24> {
        EXTI14PIN_W::new(self)
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15pin(&mut self) -> EXTI15PIN_W<28> {
        EXTI15PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ESSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [essr1](index.html) module"]
pub struct ESSR1_SPEC;
impl crate::RegisterSpec for ESSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [essr1::R](R) reader structure"]
impl crate::Readable for ESSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [essr1::W](W) writer structure"]
impl crate::Writable for ESSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESSR1 to value 0"]
impl crate::Resettable for ESSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
