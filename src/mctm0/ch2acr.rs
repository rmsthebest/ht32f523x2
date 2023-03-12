#[doc = "Register `CH2ACR` reader"]
pub struct R(crate::R<CH2ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2ACR` writer"]
pub struct W(crate::W<CH2ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2ACR_SPEC>;
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
impl From<crate::W<CH2ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2ACV` reader - CH2ACV"]
pub type CH2ACV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH2ACV` writer - CH2ACV"]
pub type CH2ACV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2ACR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    pub fn ch2acv(&self) -> CH2ACV_R {
        CH2ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch2acv(&mut self) -> CH2ACV_W<0> {
        CH2ACV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH2ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2acr](index.html) module"]
pub struct CH2ACR_SPEC;
impl crate::RegisterSpec for CH2ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2acr::R](R) reader structure"]
impl crate::Readable for CH2ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2acr::W](W) writer structure"]
impl crate::Writable for CH2ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2ACR to value 0"]
impl crate::Resettable for CH2ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
