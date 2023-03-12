#[doc = "Register `UART_URDLR` reader"]
pub struct R(crate::R<UART_URDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_URDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_URDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_URDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_URDLR` writer"]
pub struct W(crate::W<UART_URDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_URDLR_SPEC>;
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
impl From<crate::W<UART_URDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_URDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRD` reader - BRD"]
pub type BRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRD` writer - BRD"]
pub type BRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_URDLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    pub fn brd(&self) -> BRD_R {
        BRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    #[must_use]
    pub fn brd(&mut self) -> BRD_W<0> {
        BRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_URDLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urdlr](index.html) module"]
pub struct UART_URDLR_SPEC;
impl crate::RegisterSpec for UART_URDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_urdlr::R](R) reader structure"]
impl crate::Readable for UART_URDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_urdlr::W](W) writer structure"]
impl crate::Writable for UART_URDLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_URDLR to value 0"]
impl crate::Resettable for UART_URDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
