#[doc = "Register `CID3` reader"]
pub struct R(crate::R<CID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CID3` writer"]
pub struct W(crate::W<CID3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CID3_SPEC>;
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
impl From<crate::W<CID3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CID3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CID` reader - CID"]
pub type CID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CID` writer - CID"]
pub type CID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CID3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CID"]
    #[inline(always)]
    pub fn cid(&self) -> CID_R {
        CID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CID"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CID_W<0> {
        CID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CID3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](index.html) module"]
pub struct CID3_SPEC;
impl crate::RegisterSpec for CID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid3::R](R) reader structure"]
impl crate::Readable for CID3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cid3::W](W) writer structure"]
impl crate::Writable for CID3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for CID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
