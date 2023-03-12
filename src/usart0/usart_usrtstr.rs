#[doc = "Register `USART_USRTSTR` reader"]
pub struct R(crate::R<USART_USRTSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_USRTSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_USRTSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_USRTSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_USRTSTR` writer"]
pub struct W(crate::W<USART_USRTSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_USRTSTR_SPEC>;
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
impl From<crate::W<USART_USRTSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_USRTSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBM` reader - LBM"]
pub type LBM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LBM` writer - LBM"]
pub type LBM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USART_USRTSTR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<0> {
        LBM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_USRTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrtstr](index.html) module"]
pub struct USART_USRTSTR_SPEC;
impl crate::RegisterSpec for USART_USRTSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_usrtstr::R](R) reader structure"]
impl crate::Readable for USART_USRTSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_usrtstr::W](W) writer structure"]
impl crate::Writable for USART_USRTSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_USRTSTR to value 0"]
impl crate::Resettable for USART_USRTSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
