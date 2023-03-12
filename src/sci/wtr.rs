#[doc = "Register `WTR` reader"]
pub struct R(crate::R<WTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTR` writer"]
pub struct W(crate::W<WTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTR_SPEC>;
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
impl From<crate::W<WTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WT` reader - WT"]
pub type WT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WT` writer - WT"]
pub type WT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<0> {
        WT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtr](index.html) module"]
pub struct WTR_SPEC;
impl crate::RegisterSpec for WTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtr::R](R) reader structure"]
impl crate::Readable for WTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtr::W](W) writer structure"]
impl crate::Writable for WTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WTR to value 0"]
impl crate::Resettable for WTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
