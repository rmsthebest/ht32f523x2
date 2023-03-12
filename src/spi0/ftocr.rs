#[doc = "Register `FTOCR` reader"]
pub struct R(crate::R<FTOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTOCR` writer"]
pub struct W(crate::W<FTOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTOCR_SPEC>;
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
impl From<crate::W<FTOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTOCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<0> {
        TOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftocr](index.html) module"]
pub struct FTOCR_SPEC;
impl crate::RegisterSpec for FTOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftocr::R](R) reader structure"]
impl crate::Readable for FTOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftocr::W](W) writer structure"]
impl crate::Writable for FTOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTOCR to value 0"]
impl crate::Resettable for FTOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
