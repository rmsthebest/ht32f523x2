#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDR` writer"]
pub struct W(crate::W<CDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDR_SPEC>;
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
impl From<crate::W<CDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_DIV` reader - Y_DIV"]
pub type Y_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y_DIV` writer - Y_DIV"]
pub type Y_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `X_DIV` reader - X_DIV"]
pub type X_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_DIV` writer - X_DIV"]
pub type X_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `N_DIV` reader - N_DIV"]
pub type N_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `N_DIV` writer - N_DIV"]
pub type N_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    pub fn y_div(&self) -> Y_DIV_R {
        Y_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    pub fn x_div(&self) -> X_DIV_R {
        X_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    pub fn n_div(&self) -> N_DIV_R {
        N_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn y_div(&mut self) -> Y_DIV_W<0> {
        Y_DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn x_div(&mut self) -> X_DIV_W<8> {
        X_DIV_W::new(self)
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn n_div(&mut self) -> N_DIV_W<16> {
        N_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdr::W](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
