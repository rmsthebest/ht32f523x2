#[doc = "Register `OPCR` reader"]
pub struct R(crate::R<OPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPCR` writer"]
pub struct W(crate::W<OPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPCR_SPEC>;
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
impl From<crate::W<OPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPM` reader - OPM"]
pub type OPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPM` writer - OPM"]
pub type OPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<1> {
        OPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcr](index.html) module"]
pub struct OPCR_SPEC;
impl crate::RegisterSpec for OPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opcr::R](R) reader structure"]
impl crate::Readable for OPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opcr::W](W) writer structure"]
impl crate::Writable for OPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPCR to value 0"]
impl crate::Resettable for OPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
