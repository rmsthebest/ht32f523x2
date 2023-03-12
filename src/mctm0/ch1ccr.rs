#[doc = "Register `CH1CCR` reader"]
pub struct R(crate::R<CH1CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CCR` writer"]
pub struct W(crate::W<CH1CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CCR_SPEC>;
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
impl From<crate::W<CH1CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1CCV` reader - CH1CCV"]
pub type CH1CCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1CCV` writer - CH1CCV"]
pub type CH1CCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    pub fn ch1ccv(&self) -> CH1CCV_R {
        CH1CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccv(&mut self) -> CH1CCV_W<0> {
        CH1CCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH1CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ccr](index.html) module"]
pub struct CH1CCR_SPEC;
impl crate::RegisterSpec for CH1CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1ccr::R](R) reader structure"]
impl crate::Readable for CH1CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1ccr::W](W) writer structure"]
impl crate::Writable for CH1CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CCR to value 0"]
impl crate::Resettable for CH1CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
