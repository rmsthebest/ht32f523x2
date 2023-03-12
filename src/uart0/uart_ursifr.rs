#[doc = "Register `UART_URSIFR` reader"]
pub struct R(crate::R<UART_URSIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_URSIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_URSIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_URSIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_URSIFR` writer"]
pub struct W(crate::W<UART_URSIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_URSIFR_SPEC>;
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
impl From<crate::W<UART_URSIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_URSIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEI` reader - OEI"]
pub type OEI_R = crate::BitReader<bool>;
#[doc = "Field `OEI` writer - OEI"]
pub type OEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `PEI` reader - PEI"]
pub type PEI_R = crate::BitReader<bool>;
#[doc = "Field `PEI` writer - PEI"]
pub type PEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `FEI` reader - FEI"]
pub type FEI_R = crate::BitReader<bool>;
#[doc = "Field `FEI` writer - FEI"]
pub type FEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `BII` reader - BII"]
pub type BII_R = crate::BitReader<bool>;
#[doc = "Field `BII` writer - BII"]
pub type BII_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `RXDR` reader - RXDR"]
pub type RXDR_R = crate::BitReader<bool>;
#[doc = "Field `RXDR` writer - RXDR"]
pub type RXDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TXDE_R = crate::BitReader<bool>;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TXDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
#[doc = "Field `TXC` reader - TXC"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TXC"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URSIFR_SPEC, bool, O>;
impl R {
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
}
impl W {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_URSIFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ursifr](index.html) module"]
pub struct UART_URSIFR_SPEC;
impl crate::RegisterSpec for UART_URSIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ursifr::R](R) reader structure"]
impl crate::Readable for UART_URSIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ursifr::W](W) writer structure"]
impl crate::Writable for UART_URSIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_URSIFR to value 0"]
impl crate::Resettable for UART_URSIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
