#[doc = "Register `CH3ACR` reader"]
pub struct R(crate::R<CH3ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3ACR` writer"]
pub struct W(crate::W<CH3ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3ACR_SPEC>;
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
impl From<crate::W<CH3ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3ACV` reader - CH3ACV"]
pub type CH3ACV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH3ACV` writer - CH3ACV"]
pub type CH3ACV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3ACR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    pub fn ch3acv(&self) -> CH3ACV_R {
        CH3ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch3acv(&mut self) -> CH3ACV_W<0> {
        CH3ACV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH3ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3acr](index.html) module"]
pub struct CH3ACR_SPEC;
impl crate::RegisterSpec for CH3ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3acr::R](R) reader structure"]
impl crate::Readable for CH3ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3acr::W](W) writer structure"]
impl crate::Writable for CH3ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3ACR to value 0"]
impl crate::Resettable for CH3ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
