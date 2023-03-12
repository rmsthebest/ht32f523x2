#[doc = "Register `UART_URDR` reader"]
pub struct R(crate::R<UART_URDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_URDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_URDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_URDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_URDR` writer"]
pub struct W(crate::W<UART_URDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_URDR_SPEC>;
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
impl From<crate::W<UART_URDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_URDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB` reader - DB"]
pub type DB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DB` writer - DB"]
pub type DB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_URDR_SPEC, u16, u16, 9, O>;
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
#[doc = "UART_URDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urdr](index.html) module"]
pub struct UART_URDR_SPEC;
impl crate::RegisterSpec for UART_URDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_urdr::R](R) reader structure"]
impl crate::Readable for UART_URDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_urdr::W](W) writer structure"]
impl crate::Writable for UART_URDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_URDR to value 0"]
impl crate::Resettable for UART_URDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
