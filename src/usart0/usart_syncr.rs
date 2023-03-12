#[doc = "Register `USART_SYNCR` reader"]
pub struct R(crate::R<USART_SYNCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_SYNCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_SYNCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_SYNCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_SYNCR` writer"]
pub struct W(crate::W<USART_SYNCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_SYNCR_SPEC>;
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
impl From<crate::W<USART_SYNCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_SYNCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKEN` reader - CLKEN"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - CLKEN"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_SYNCR_SPEC, bool, O>;
#[doc = "Field `CPS` reader - CPS"]
pub type CPS_R = crate::BitReader<bool>;
#[doc = "Field `CPS` writer - CPS"]
pub type CPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_SYNCR_SPEC, bool, O>;
#[doc = "Field `CPO` reader - CPO"]
pub type CPO_R = crate::BitReader<bool>;
#[doc = "Field `CPO` writer - CPO"]
pub type CPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_SYNCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    pub fn cpo(&self) -> CPO_R {
        CPO_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    #[must_use]
    pub fn cps(&mut self) -> CPS_W<2> {
        CPS_W::new(self)
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    #[must_use]
    pub fn cpo(&mut self) -> CPO_W<3> {
        CPO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_SYNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_syncr](index.html) module"]
pub struct USART_SYNCR_SPEC;
impl crate::RegisterSpec for USART_SYNCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_syncr::R](R) reader structure"]
impl crate::Readable for USART_SYNCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_syncr::W](W) writer structure"]
impl crate::Writable for USART_SYNCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_SYNCR to value 0"]
impl crate::Resettable for USART_SYNCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
