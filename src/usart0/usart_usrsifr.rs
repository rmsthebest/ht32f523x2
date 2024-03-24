#[doc = "Register `USART_USRSIFR` reader"]
pub type R = crate::R<UsartUsrsifrSpec>;
#[doc = "Register `USART_USRSIFR` writer"]
pub type W = crate::W<UsartUsrsifrSpec>;
#[doc = "Field `RXDNE` reader - RXDNE"]
pub type RxdneR = crate::BitReader;
#[doc = "Field `RXDNE` writer - RXDNE"]
pub type RxdneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEI` reader - OEI"]
pub type OeiR = crate::BitReader;
#[doc = "Field `OEI` writer - OEI"]
pub type OeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEI` reader - PEI"]
pub type PeiR = crate::BitReader;
#[doc = "Field `PEI` writer - PEI"]
pub type PeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEI` reader - FEI"]
pub type FeiR = crate::BitReader;
#[doc = "Field `FEI` writer - FEI"]
pub type FeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BII` reader - BII"]
pub type BiiR = crate::BitReader;
#[doc = "Field `BII` writer - BII"]
pub type BiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDR` reader - RXDR"]
pub type RxdrR = crate::BitReader;
#[doc = "Field `RXDR` writer - RXDR"]
pub type RxdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTOF` reader - RXTOF"]
pub type RxtofR = crate::BitReader;
#[doc = "Field `RXTOF` writer - RXTOF"]
pub type RxtofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TxdeR = crate::BitReader;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TxdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - TXC"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TXC"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSADDE` reader - RSADDE"]
pub type RsaddeR = crate::BitReader;
#[doc = "Field `RSADDE` writer - RSADDE"]
pub type RsaddeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSC` reader - CTSC"]
pub type CtscR = crate::BitReader;
#[doc = "Field `CTSC` writer - CTSC"]
pub type CtscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSS` reader - CTSS"]
pub type CtssR = crate::BitReader;
#[doc = "Field `CTSS` writer - CTSS"]
pub type CtssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RxdneR {
        RxdneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OeiR {
        OeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&self) -> PeiR {
        PeiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&self) -> FeiR {
        FeiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&self) -> BiiR {
        BiiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    pub fn rxtof(&self) -> RxtofR {
        RxtofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TxdeR {
        TxdeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    pub fn rsadde(&self) -> RsaddeR {
        RsaddeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    pub fn ctsc(&self) -> CtscR {
        CtscR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    pub fn ctss(&self) -> CtssR {
        CtssR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdne(&mut self) -> RxdneW<UsartUsrsifrSpec> {
        RxdneW::new(self, 0)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    #[must_use]
    pub fn oei(&mut self) -> OeiW<UsartUsrsifrSpec> {
        OeiW::new(self, 1)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    #[must_use]
    pub fn pei(&mut self) -> PeiW<UsartUsrsifrSpec> {
        PeiW::new(self, 2)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    #[must_use]
    pub fn fei(&mut self) -> FeiW<UsartUsrsifrSpec> {
        FeiW::new(self, 3)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    #[must_use]
    pub fn bii(&mut self) -> BiiW<UsartUsrsifrSpec> {
        BiiW::new(self, 4)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    #[must_use]
    pub fn rxdr(&mut self) -> RxdrW<UsartUsrsifrSpec> {
        RxdrW::new(self, 5)
    }
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    #[must_use]
    pub fn rxtof(&mut self) -> RxtofW<UsartUsrsifrSpec> {
        RxtofW::new(self, 6)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    #[must_use]
    pub fn txde(&mut self) -> TxdeW<UsartUsrsifrSpec> {
        TxdeW::new(self, 7)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<UsartUsrsifrSpec> {
        TxcW::new(self, 8)
    }
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    #[must_use]
    pub fn rsadde(&mut self) -> RsaddeW<UsartUsrsifrSpec> {
        RsaddeW::new(self, 9)
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    #[must_use]
    pub fn ctsc(&mut self) -> CtscW<UsartUsrsifrSpec> {
        CtscW::new(self, 10)
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    #[must_use]
    pub fn ctss(&mut self) -> CtssW<UsartUsrsifrSpec> {
        CtssW::new(self, 11)
    }
}
#[doc = "USART_USRSIFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrsifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrsifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrsifrSpec;
impl crate::RegisterSpec for UsartUsrsifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrsifr::R`](R) reader structure"]
impl crate::Readable for UsartUsrsifrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrsifr::W`](W) writer structure"]
impl crate::Writable for UsartUsrsifrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRSIFR to value 0"]
impl crate::Resettable for UsartUsrsifrSpec {
    const RESET_VALUE: u32 = 0;
}
