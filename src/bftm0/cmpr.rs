#[doc = "Register `CMPR` reader"]
pub struct R(crate::R<CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR` writer"]
pub struct W(crate::W<CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR_SPEC>;
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
impl From<crate::W<CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - CMP"]
pub type CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMP` writer - CMP"]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr](index.html) module"]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr::R](R) reader structure"]
impl crate::Readable for CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr::W](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
