#[doc = "Register `CH2CCR` reader"]
pub struct R(crate::R<CH2CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CCR` writer"]
pub struct W(crate::W<CH2CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CCR_SPEC>;
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
impl From<crate::W<CH2CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2CCV` reader - CH2CCV"]
pub type CH2CCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH2CCV` writer - CH2CCV"]
pub type CH2CCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2CCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&self) -> CH2CCV_R {
        CH2CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccv(&mut self) -> CH2CCV_W<0> {
        CH2CCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH2CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ccr](index.html) module"]
pub struct CH2CCR_SPEC;
impl crate::RegisterSpec for CH2CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2ccr::R](R) reader structure"]
impl crate::Readable for CH2CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2ccr::W](W) writer structure"]
impl crate::Writable for CH2CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CCR to value 0"]
impl crate::Resettable for CH2CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
