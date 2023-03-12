#[doc = "Register `GTR` reader"]
pub struct R(crate::R<GTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTR` writer"]
pub struct W(crate::W<GTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTR_SPEC>;
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
impl From<crate::W<GTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GT` reader - GT"]
pub type GT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GT` writer - GT"]
pub type GT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<0> {
        GT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtr](index.html) module"]
pub struct GTR_SPEC;
impl crate::RegisterSpec for GTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtr::R](R) reader structure"]
impl crate::Readable for GTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtr::W](W) writer structure"]
impl crate::Writable for GTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTR to value 0"]
impl crate::Resettable for GTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
