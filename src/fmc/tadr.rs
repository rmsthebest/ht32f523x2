#[doc = "Register `TADR` reader"]
pub struct R(crate::R<TADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TADR` writer"]
pub struct W(crate::W<TADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TADR_SPEC>;
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
impl From<crate::W<TADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TADB` reader - TADB"]
pub type TADB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TADB` writer - TADB"]
pub type TADB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TADR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TADB"]
    #[inline(always)]
    pub fn tadb(&self) -> TADB_R {
        TADB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TADB"]
    #[inline(always)]
    #[must_use]
    pub fn tadb(&mut self) -> TADB_W<0> {
        TADB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tadr](index.html) module"]
pub struct TADR_SPEC;
impl crate::RegisterSpec for TADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tadr::R](R) reader structure"]
impl crate::Readable for TADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tadr::W](W) writer structure"]
impl crate::Writable for TADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TADR to value 0"]
impl crate::Resettable for TADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
