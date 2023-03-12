#[doc = "Register `TXB` reader"]
pub struct R(crate::R<TXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXB` writer"]
pub struct W(crate::W<TXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXB_SPEC>;
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
impl From<crate::W<TXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TB` reader - TB"]
pub type TB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TB` writer - TB"]
pub type TB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TB_W<0> {
        TB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txb](index.html) module"]
pub struct TXB_SPEC;
impl crate::RegisterSpec for TXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txb::R](R) reader structure"]
impl crate::Readable for TXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txb::W](W) writer structure"]
impl crate::Writable for TXB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXB to value 0"]
impl crate::Resettable for TXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
