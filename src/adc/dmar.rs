#[doc = "Register `DMAR` reader"]
pub struct R(crate::R<DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAR` writer"]
pub struct W(crate::W<DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAR_SPEC>;
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
impl From<crate::W<DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMAS` reader - ADDMAS"]
pub type ADDMAS_R = crate::BitReader<bool>;
#[doc = "Field `ADDMAS` writer - ADDMAS"]
pub type ADDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAR_SPEC, bool, O>;
#[doc = "Field `ADDMAG` reader - ADDMAG"]
pub type ADDMAG_R = crate::BitReader<bool>;
#[doc = "Field `ADDMAG` writer - ADDMAG"]
pub type ADDMAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAR_SPEC, bool, O>;
#[doc = "Field `ADDMAC` reader - ADDMAC"]
pub type ADDMAC_R = crate::BitReader<bool>;
#[doc = "Field `ADDMAC` writer - ADDMAC"]
pub type ADDMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&self) -> ADDMAS_R {
        ADDMAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&self) -> ADDMAG_R {
        ADDMAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&self) -> ADDMAC_R {
        ADDMAC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    #[must_use]
    pub fn addmas(&mut self) -> ADDMAS_W<0> {
        ADDMAS_W::new(self)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    #[must_use]
    pub fn addmag(&mut self) -> ADDMAG_W<1> {
        ADDMAG_W::new(self)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn addmac(&mut self) -> ADDMAC_W<2> {
        ADDMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](index.html) module"]
pub struct DMAR_SPEC;
impl crate::RegisterSpec for DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmar::R](R) reader structure"]
impl crate::Readable for DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmar::W](W) writer structure"]
impl crate::Writable for DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
