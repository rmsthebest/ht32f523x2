#[doc = "Register `ICLR` reader"]
pub struct R(crate::R<ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADICLRS` reader - ADICLRS"]
pub type ADICLRS_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRS` writer - ADICLRS"]
pub type ADICLRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `ADICLRG` reader - ADICLRG"]
pub type ADICLRG_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRG` writer - ADICLRG"]
pub type ADICLRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `ADICLRC` reader - ADICLRC"]
pub type ADICLRC_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRC` writer - ADICLRC"]
pub type ADICLRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `ADICLRL` reader - ADICLRL"]
pub type ADICLRL_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRL` writer - ADICLRL"]
pub type ADICLRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `ADICLRU` reader - ADICLRU"]
pub type ADICLRU_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRU` writer - ADICLRU"]
pub type ADICLRU_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `ADICLRO` reader - ADICLRO"]
pub type ADICLRO_R = crate::BitReader<bool>;
#[doc = "Field `ADICLRO` writer - ADICLRO"]
pub type ADICLRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&self) -> ADICLRS_R {
        ADICLRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&self) -> ADICLRG_R {
        ADICLRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&self) -> ADICLRC_R {
        ADICLRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&self) -> ADICLRL_R {
        ADICLRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&self) -> ADICLRU_R {
        ADICLRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&self) -> ADICLRO_R {
        ADICLRO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrs(&mut self) -> ADICLRS_W<0> {
        ADICLRS_W::new(self)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrg(&mut self) -> ADICLRG_W<1> {
        ADICLRG_W::new(self)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrc(&mut self) -> ADICLRC_W<2> {
        ADICLRC_W::new(self)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    #[must_use]
    pub fn adiclrl(&mut self) -> ADICLRL_W<16> {
        ADICLRL_W::new(self)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    #[must_use]
    pub fn adiclru(&mut self) -> ADICLRU_W<17> {
        ADICLRU_W::new(self)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    #[must_use]
    pub fn adiclro(&mut self) -> ADICLRO_W<24> {
        ADICLRO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclr::R](R) reader structure"]
impl crate::Readable for ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
