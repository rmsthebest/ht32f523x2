#[doc = "Register `OCMR` reader"]
pub struct R(crate::R<OCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMR` writer"]
pub struct W(crate::W<OCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMR_SPEC>;
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
impl From<crate::W<OCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - CMD"]
pub type CMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD` writer - CMD"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCMR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmr](index.html) module"]
pub struct OCMR_SPEC;
impl crate::RegisterSpec for OCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocmr::R](R) reader structure"]
impl crate::Readable for OCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocmr::W](W) writer structure"]
impl crate::Writable for OCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMR to value 0"]
impl crate::Resettable for OCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
