#[doc = "Register `ADDMR` reader"]
pub struct R(crate::R<ADDMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDMR` writer"]
pub struct W(crate::W<ADDMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDMR_SPEC>;
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
impl From<crate::W<ADDMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMR` reader - ADDMR"]
pub type ADDMR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDMR` writer - ADDMR"]
pub type ADDMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDMR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    pub fn addmr(&self) -> ADDMR_R {
        ADDMR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    #[must_use]
    pub fn addmr(&mut self) -> ADDMR_W<0> {
        ADDMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADDMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addmr](index.html) module"]
pub struct ADDMR_SPEC;
impl crate::RegisterSpec for ADDMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addmr::R](R) reader structure"]
impl crate::Readable for ADDMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addmr::W](W) writer structure"]
impl crate::Writable for ADDMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDMR to value 0"]
impl crate::Resettable for ADDMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
