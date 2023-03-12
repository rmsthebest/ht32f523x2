#[doc = "Register `PNSR` reader"]
pub struct R(crate::R<PNSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PNSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PNSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PNSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PNSR` writer"]
pub struct W(crate::W<PNSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PNSR_SPEC>;
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
impl From<crate::W<PNSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PNSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PNSB` reader - PNSB"]
pub type PNSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PNSB` writer - PNSB"]
pub type PNSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PNSR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    pub fn pnsb(&self) -> PNSB_R {
        PNSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    #[must_use]
    pub fn pnsb(&mut self) -> PNSB_W<0> {
        PNSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PNSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pnsr](index.html) module"]
pub struct PNSR_SPEC;
impl crate::RegisterSpec for PNSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pnsr::R](R) reader structure"]
impl crate::Readable for PNSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pnsr::W](W) writer structure"]
impl crate::Writable for PNSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PNSR to value 0"]
impl crate::Resettable for PNSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
