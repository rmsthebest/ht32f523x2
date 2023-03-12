#[doc = "Register `TOUT` reader"]
pub struct R(crate::R<TOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUT` writer"]
pub struct W(crate::W<TOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUT_SPEC>;
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
impl From<crate::W<TOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUT` reader - TOUT"]
pub type TOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUT` writer - TOUT"]
pub type TOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUT_SPEC, u16, u16, 16, O>;
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<0> {
        TOUT_W::new(self)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<16> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tout](index.html) module"]
pub struct TOUT_SPEC;
impl crate::RegisterSpec for TOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tout::R](R) reader structure"]
impl crate::Readable for TOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tout::W](W) writer structure"]
impl crate::Writable for TOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUT to value 0"]
impl crate::Resettable for TOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
