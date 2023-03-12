#[doc = "Register `USART_USRSIFR` reader"]
pub struct R(crate::R<USART_USRSIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_USRSIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_USRSIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_USRSIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_USRSIFR` writer"]
pub struct W(crate::W<USART_USRSIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_USRSIFR_SPEC>;
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
impl From<crate::W<USART_USRSIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_USRSIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDNE` reader - RXDNE"]
pub type RXDNE_R = crate::BitReader<bool>;
#[doc = "Field `RXDNE` writer - RXDNE"]
pub type RXDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `OEI` reader - OEI"]
pub type OEI_R = crate::BitReader<bool>;
#[doc = "Field `OEI` writer - OEI"]
pub type OEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `PEI` reader - PEI"]
pub type PEI_R = crate::BitReader<bool>;
#[doc = "Field `PEI` writer - PEI"]
pub type PEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `FEI` reader - FEI"]
pub type FEI_R = crate::BitReader<bool>;
#[doc = "Field `FEI` writer - FEI"]
pub type FEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `BII` reader - BII"]
pub type BII_R = crate::BitReader<bool>;
#[doc = "Field `BII` writer - BII"]
pub type BII_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `RXDR` reader - RXDR"]
pub type RXDR_R = crate::BitReader<bool>;
#[doc = "Field `RXDR` writer - RXDR"]
pub type RXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `RXTOF` reader - RXTOF"]
pub type RXTOF_R = crate::BitReader<bool>;
#[doc = "Field `RXTOF` writer - RXTOF"]
pub type RXTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TXDE_R = crate::BitReader<bool>;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TXDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `TXC` reader - TXC"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TXC"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `RSADDE` reader - RSADDE"]
pub type RSADDE_R = crate::BitReader<bool>;
#[doc = "Field `RSADDE` writer - RSADDE"]
pub type RSADDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `CTSC` reader - CTSC"]
pub type CTSC_R = crate::BitReader<bool>;
#[doc = "Field `CTSC` writer - CTSC"]
pub type CTSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
#[doc = "Field `CTSS` reader - CTSS"]
pub type CTSS_R = crate::BitReader<bool>;
#[doc = "Field `CTSS` writer - CTSS"]
pub type CTSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_USRSIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RXDNE_R {
        RXDNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OEI_R {
        OEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&self) -> PEI_R {
        PEI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&self) -> FEI_R {
        FEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&self) -> BII_R {
        BII_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    pub fn rxtof(&self) -> RXTOF_R {
        RXTOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    pub fn rsadde(&self) -> RSADDE_R {
        RSADDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    pub fn ctsc(&self) -> CTSC_R {
        CTSC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    pub fn ctss(&self) -> CTSS_R {
        CTSS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdne(&mut self) -> RXDNE_W<0> {
        RXDNE_W::new(self)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    #[must_use]
    pub fn oei(&mut self) -> OEI_W<1> {
        OEI_W::new(self)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    #[must_use]
    pub fn pei(&mut self) -> PEI_W<2> {
        PEI_W::new(self)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    #[must_use]
    pub fn fei(&mut self) -> FEI_W<3> {
        FEI_W::new(self)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    #[must_use]
    pub fn bii(&mut self) -> BII_W<4> {
        BII_W::new(self)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    #[must_use]
    pub fn rxdr(&mut self) -> RXDR_W<5> {
        RXDR_W::new(self)
    }
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    #[must_use]
    pub fn rxtof(&mut self) -> RXTOF_W<6> {
        RXTOF_W::new(self)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    #[must_use]
    pub fn txde(&mut self) -> TXDE_W<7> {
        TXDE_W::new(self)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<8> {
        TXC_W::new(self)
    }
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    #[must_use]
    pub fn rsadde(&mut self) -> RSADDE_W<9> {
        RSADDE_W::new(self)
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    #[must_use]
    pub fn ctsc(&mut self) -> CTSC_W<10> {
        CTSC_W::new(self)
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    #[must_use]
    pub fn ctss(&mut self) -> CTSS_W<11> {
        CTSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_USRSIFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrsifr](index.html) module"]
pub struct USART_USRSIFR_SPEC;
impl crate::RegisterSpec for USART_USRSIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_usrsifr::R](R) reader structure"]
impl crate::Readable for USART_USRSIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_usrsifr::W](W) writer structure"]
impl crate::Writable for USART_USRSIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_USRSIFR to value 0"]
impl crate::Resettable for USART_USRSIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
