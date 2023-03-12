#[doc = "Register `USART_USRDLR` reader"]
pub struct R(crate::R<USART_USRDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_USRDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_USRDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_USRDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_USRDLR` writer"]
pub struct W(crate::W<USART_USRDLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_USRDLR_SPEC>;
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
impl From<crate::W<USART_USRDLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_USRDLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRD` reader - BRD"]
pub type BRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRD` writer - BRD"]
pub type BRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USART_USRDLR_SPEC, u16, u16, 16, O>;
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
#[doc = "USART_USRDLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrdlr](index.html) module"]
pub struct USART_USRDLR_SPEC;
impl crate::RegisterSpec for USART_USRDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_usrdlr::R](R) reader structure"]
impl crate::Readable for USART_USRDLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_usrdlr::W](W) writer structure"]
impl crate::Writable for USART_USRDLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_USRDLR to value 0"]
impl crate::Resettable for USART_USRDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
