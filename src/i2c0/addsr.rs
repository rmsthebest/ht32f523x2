#[doc = "Register `ADDSR` reader"]
pub struct R(crate::R<ADDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDSR` writer"]
pub struct W(crate::W<ADDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDSR_SPEC>;
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
impl From<crate::W<ADDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDSR` reader - ADDSR"]
pub type ADDSR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDSR` writer - ADDSR"]
pub type ADDSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDSR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    pub fn addsr(&self) -> ADDSR_R {
        ADDSR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    #[must_use]
    pub fn addsr(&mut self) -> ADDSR_W<0> {
        ADDSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADDSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addsr](index.html) module"]
pub struct ADDSR_SPEC;
impl crate::RegisterSpec for ADDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addsr::R](R) reader structure"]
impl crate::Readable for ADDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addsr::W](W) writer structure"]
impl crate::Writable for ADDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDSR to value 0"]
impl crate::Resettable for ADDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
