#[doc = "Register `ETUR` reader"]
pub struct R(crate::R<ETUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETUR` writer"]
pub struct W(crate::W<ETUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETUR_SPEC>;
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
impl From<crate::W<ETUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETU` reader - ETU"]
pub type ETU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ETU` writer - ETU"]
pub type ETU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETUR_SPEC, u16, u16, 11, O>;
#[doc = "Field `COMP` reader - COMP"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - COMP"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    pub fn etu(&self) -> ETU_R {
        ETU_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    #[must_use]
    pub fn etu(&mut self) -> ETU_W<0> {
        ETU_W::new(self)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<15> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etur](index.html) module"]
pub struct ETUR_SPEC;
impl crate::RegisterSpec for ETUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etur::R](R) reader structure"]
impl crate::Readable for ETUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etur::W](W) writer structure"]
impl crate::Writable for ETUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETUR to value 0"]
impl crate::Resettable for ETUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
