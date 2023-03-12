#[doc = "Register `USART_USRDR` reader"]
pub struct R(crate::R<USART_USRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_USRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_USRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_USRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_USRDR` writer"]
pub struct W(crate::W<USART_USRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_USRDR_SPEC>;
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
impl From<crate::W<USART_USRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_USRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB` reader - DB"]
pub type DB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DB` writer - DB"]
pub type DB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USART_USRDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    #[must_use]
    pub fn db(&mut self) -> DB_W<0> {
        DB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_USRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrdr](index.html) module"]
pub struct USART_USRDR_SPEC;
impl crate::RegisterSpec for USART_USRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_usrdr::R](R) reader structure"]
impl crate::Readable for USART_USRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_usrdr::W](W) writer structure"]
impl crate::Writable for USART_USRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_USRDR to value 0"]
impl crate::Resettable for USART_USRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
