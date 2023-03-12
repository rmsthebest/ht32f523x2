#[doc = "Register `PLLCR` reader"]
pub struct R(crate::R<PLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCR` writer"]
pub struct W(crate::W<PLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCR_SPEC>;
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
impl From<crate::W<PLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLBPS` reader - PLLBPS"]
pub type PLLBPS_R = crate::BitReader<bool>;
#[doc = "Field `PLLBPS` writer - PLLBPS"]
pub type PLLBPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    pub fn pllbps(&self) -> PLLBPS_R {
        PLLBPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    #[must_use]
    pub fn pllbps(&mut self) -> PLLBPS_W<31> {
        PLLBPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcr](index.html) module"]
pub struct PLLCR_SPEC;
impl crate::RegisterSpec for PLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcr::R](R) reader structure"]
impl crate::Readable for PLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcr::W](W) writer structure"]
impl crate::Writable for PLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCR to value 0"]
impl crate::Resettable for PLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
