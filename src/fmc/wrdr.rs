#[doc = "Register `WRDR` reader"]
pub struct R(crate::R<WRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRDR` writer"]
pub struct W(crate::W<WRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRDR_SPEC>;
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
impl From<crate::W<WRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRDB` reader - WRDB"]
pub type WRDB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRDB` writer - WRDB"]
pub type WRDB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    pub fn wrdb(&self) -> WRDB_R {
        WRDB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    #[must_use]
    pub fn wrdb(&mut self) -> WRDB_W<0> {
        WRDB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrdr](index.html) module"]
pub struct WRDR_SPEC;
impl crate::RegisterSpec for WRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrdr::R](R) reader structure"]
impl crate::Readable for WRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrdr::W](W) writer structure"]
impl crate::Writable for WRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRDR to value 0"]
impl crate::Resettable for WRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
