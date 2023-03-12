#[doc = "Register `USART_USRTPR` reader"]
pub struct R(crate::R<USART_USRTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_USRTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_USRTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_USRTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_USRTPR` writer"]
pub struct W(crate::W<USART_USRTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_USRTPR_SPEC>;
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
impl From<crate::W<USART_USRTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_USRTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTOC` reader - RXTOC"]
pub type RXTOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTOC` writer - RXTOC"]
pub type RXTOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USART_USRTPR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RXTOEN` reader - RXTOEN"]
pub type RXTOEN_R = crate::BitReader<bool>;
#[doc = "Field `RXTOEN` writer - RXTOEN"]
pub type RXTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRTPR_SPEC, bool, O>;
#[doc = "Field `TG` reader - TG"]
pub type TG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TG` writer - TG"]
pub type TG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USART_USRTPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    pub fn rxtoc(&self) -> RXTOC_R {
        RXTOC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    pub fn rxtoen(&self) -> RXTOEN_R {
        RXTOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoc(&mut self) -> RXTOC_W<0> {
        RXTOC_W::new(self)
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoen(&mut self) -> RXTOEN_W<7> {
        RXTOEN_W::new(self)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<8> {
        TG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_USRTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrtpr](index.html) module"]
pub struct USART_USRTPR_SPEC;
impl crate::RegisterSpec for USART_USRTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_usrtpr::R](R) reader structure"]
impl crate::Readable for USART_USRTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_usrtpr::W](W) writer structure"]
impl crate::Writable for USART_USRTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_USRTPR to value 0"]
impl crate::Resettable for USART_USRTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
