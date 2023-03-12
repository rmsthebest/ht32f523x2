#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV` reader - CONV"]
pub type CONV_R = crate::BitReader<bool>;
#[doc = "Field `CONV` writer - CONV"]
pub type CONV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CREP` reader - CREP"]
pub type CREP_R = crate::BitReader<bool>;
#[doc = "Field `CREP` writer - CREP"]
pub type CREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WTEN` reader - WTEN"]
pub type WTEN_R = crate::BitReader<bool>;
#[doc = "Field `WTEN` writer - WTEN"]
pub type WTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SCIM` reader - SCIM"]
pub type SCIM_R = crate::BitReader<bool>;
#[doc = "Field `SCIM` writer - SCIM"]
pub type SCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RETRY` reader - RETRY"]
pub type RETRY_R = crate::BitReader<bool>;
#[doc = "Field `RETRY` writer - RETRY"]
pub type RETRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ENSCI` reader - ENSCI"]
pub type ENSCI_R = crate::BitReader<bool>;
#[doc = "Field `ENSCI` writer - ENSCI"]
pub type ENSCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DETCNF` reader - DETCNF"]
pub type DETCNF_R = crate::BitReader<bool>;
#[doc = "Field `DETCNF` writer - DETCNF"]
pub type DETCNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXDMA` reader - TXDMA"]
pub type TXDMA_R = crate::BitReader<bool>;
#[doc = "Field `TXDMA` writer - TXDMA"]
pub type TXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXDMA` reader - RXDMA"]
pub type RXDMA_R = crate::BitReader<bool>;
#[doc = "Field `RXDMA` writer - RXDMA"]
pub type RXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    pub fn wten(&self) -> WTEN_R {
        WTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    pub fn scim(&self) -> SCIM_R {
        SCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    pub fn ensci(&self) -> ENSCI_R {
        ENSCI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    pub fn detcnf(&self) -> DETCNF_R {
        DETCNF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    #[must_use]
    pub fn conv(&mut self) -> CONV_W<0> {
        CONV_W::new(self)
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CREP_W<1> {
        CREP_W::new(self)
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wten(&mut self) -> WTEN_W<2> {
        WTEN_W::new(self)
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    #[must_use]
    pub fn scim(&mut self) -> SCIM_W<3> {
        SCIM_W::new(self)
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RETRY_W<4> {
        RETRY_W::new(self)
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    #[must_use]
    pub fn ensci(&mut self) -> ENSCI_W<5> {
        ENSCI_W::new(self)
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    #[must_use]
    pub fn detcnf(&mut self) -> DETCNF_W<6> {
        DETCNF_W::new(self)
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TXDMA_W<8> {
        TXDMA_W::new(self)
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RXDMA_W<9> {
        RXDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
