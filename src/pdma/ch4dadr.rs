#[doc = "Register `CH4DADR` reader"]
pub struct R(crate::R<CH4DADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4DADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4DADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4DADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4DADR` writer"]
pub struct W(crate::W<CH4DADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4DADR_SPEC>;
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
impl From<crate::W<CH4DADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4DADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADR` reader - DADR"]
pub type DADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DADR` writer - DADR"]
pub type DADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH4DADR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DADR_W<0> {
        DADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH4DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4dadr](index.html) module"]
pub struct CH4DADR_SPEC;
impl crate::RegisterSpec for CH4DADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4dadr::R](R) reader structure"]
impl crate::Readable for CH4DADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4dadr::W](W) writer structure"]
impl crate::Writable for CH4DADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4DADR to value 0"]
impl crate::Resettable for CH4DADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
