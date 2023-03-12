#[doc = "Register `MR1` reader"]
pub struct R(crate::R<MR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR1` writer"]
pub struct W(crate::W<MR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR1_SPEC>;
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
impl From<crate::W<MR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTD` reader - WDTD"]
pub type WDTD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDTD` writer - WDTD"]
pub type WDTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR1_SPEC, u16, u16, 12, O>;
#[doc = "Field `WPSC` reader - WPSC"]
pub type WPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WPSC` writer - WPSC"]
pub type WPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    pub fn wdtd(&self) -> WDTD_R {
        WDTD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    pub fn wpsc(&self) -> WPSC_R {
        WPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    #[must_use]
    pub fn wdtd(&mut self) -> WDTD_W<0> {
        WDTD_W::new(self)
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    #[must_use]
    pub fn wpsc(&mut self) -> WPSC_W<12> {
        WPSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr1](index.html) module"]
pub struct MR1_SPEC;
impl crate::RegisterSpec for MR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr1::R](R) reader structure"]
impl crate::Readable for MR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr1::W](W) writer structure"]
impl crate::Writable for MR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR1 to value 0"]
impl crate::Resettable for MR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
