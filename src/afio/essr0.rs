#[doc = "Register `ESSR0` reader"]
pub struct R(crate::R<ESSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESSR0` writer"]
pub struct W(crate::W<ESSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESSR0_SPEC>;
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
impl From<crate::W<ESSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0PIN` reader - EXTI0PIN"]
pub type EXTI0PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0PIN` writer - EXTI0PIN"]
pub type EXTI0PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI1PIN` reader - EXTI1PIN"]
pub type EXTI1PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI1PIN` writer - EXTI1PIN"]
pub type EXTI1PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI2PIN` reader - EXTI2PIN"]
pub type EXTI2PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI2PIN` writer - EXTI2PIN"]
pub type EXTI2PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI3PIN` reader - EXTI3PIN"]
pub type EXTI3PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI3PIN` writer - EXTI3PIN"]
pub type EXTI3PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI4PIN` reader - EXTI4PIN"]
pub type EXTI4PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI4PIN` writer - EXTI4PIN"]
pub type EXTI4PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI5PIN` reader - EXTI5PIN"]
pub type EXTI5PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI5PIN` writer - EXTI5PIN"]
pub type EXTI5PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI6PIN` reader - EXTI6PIN"]
pub type EXTI6PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI6PIN` writer - EXTI6PIN"]
pub type EXTI6PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI7PIN` reader - EXTI7PIN"]
pub type EXTI7PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI7PIN` writer - EXTI7PIN"]
pub type EXTI7PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESSR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    pub fn exti0pin(&self) -> EXTI0PIN_R {
        EXTI0PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    pub fn exti1pin(&self) -> EXTI1PIN_R {
        EXTI1PIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    pub fn exti2pin(&self) -> EXTI2PIN_R {
        EXTI2PIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    pub fn exti3pin(&self) -> EXTI3PIN_R {
        EXTI3PIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    pub fn exti4pin(&self) -> EXTI4PIN_R {
        EXTI4PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    pub fn exti5pin(&self) -> EXTI5PIN_R {
        EXTI5PIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    pub fn exti6pin(&self) -> EXTI6PIN_R {
        EXTI6PIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    pub fn exti7pin(&self) -> EXTI7PIN_R {
        EXTI7PIN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0pin(&mut self) -> EXTI0PIN_W<0> {
        EXTI0PIN_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1pin(&mut self) -> EXTI1PIN_W<4> {
        EXTI1PIN_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2pin(&mut self) -> EXTI2PIN_W<8> {
        EXTI2PIN_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3pin(&mut self) -> EXTI3PIN_W<12> {
        EXTI3PIN_W::new(self)
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4pin(&mut self) -> EXTI4PIN_W<16> {
        EXTI4PIN_W::new(self)
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5pin(&mut self) -> EXTI5PIN_W<20> {
        EXTI5PIN_W::new(self)
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6pin(&mut self) -> EXTI6PIN_W<24> {
        EXTI6PIN_W::new(self)
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7pin(&mut self) -> EXTI7PIN_W<28> {
        EXTI7PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ESSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [essr0](index.html) module"]
pub struct ESSR0_SPEC;
impl crate::RegisterSpec for ESSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [essr0::R](R) reader structure"]
impl crate::Readable for ESSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [essr0::W](W) writer structure"]
impl crate::Writable for ESSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESSR0 to value 0"]
impl crate::Resettable for ESSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
