#[doc = "Register `EP2TCR` reader"]
pub struct R(crate::R<EP2TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP2TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP2TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP2TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP2TCR` writer"]
pub struct W(crate::W<EP2TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP2TCR_SPEC>;
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
impl From<crate::W<EP2TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP2TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT` reader - TCNT"]
pub type TCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TCNT` writer - TCNT"]
pub type TCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP2TCR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    pub fn tcnt(&self) -> TCNT_R {
        TCNT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt(&mut self) -> TCNT_W<0> {
        TCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP2TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2tcr](index.html) module"]
pub struct EP2TCR_SPEC;
impl crate::RegisterSpec for EP2TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep2tcr::R](R) reader structure"]
impl crate::Readable for EP2TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep2tcr::W](W) writer structure"]
impl crate::Writable for EP2TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP2TCR to value 0"]
impl crate::Resettable for EP2TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
