#[doc = "Register `RXB` reader"]
pub struct R(crate::R<RXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXB` writer"]
pub struct W(crate::W<RXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXB_SPEC>;
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
impl From<crate::W<RXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - RB"]
pub type RB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB` writer - RB"]
pub type RB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<0> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxb](index.html) module"]
pub struct RXB_SPEC;
impl crate::RegisterSpec for RXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxb::R](R) reader structure"]
impl crate::Readable for RXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxb::W](W) writer structure"]
impl crate::Writable for RXB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXB to value 0"]
impl crate::Resettable for RXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
