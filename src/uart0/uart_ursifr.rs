#[doc = "Register `UART_URSIFR` reader"]
pub type R = crate::R<UartUrsifrSpec>;
#[doc = "Register `UART_URSIFR` writer"]
pub type W = crate::W<UartUrsifrSpec>;
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
#[doc = "Field `TXDE` reader - TXDE"]
pub type TxdeR = crate::BitReader;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TxdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - TXC"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TXC"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
}
impl W {
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    #[must_use]
    pub fn oei(&mut self) -> OeiW<UartUrsifrSpec> {
        OeiW::new(self, 1)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    #[must_use]
    pub fn pei(&mut self) -> PeiW<UartUrsifrSpec> {
        PeiW::new(self, 2)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    #[must_use]
    pub fn fei(&mut self) -> FeiW<UartUrsifrSpec> {
        FeiW::new(self, 3)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    #[must_use]
    pub fn bii(&mut self) -> BiiW<UartUrsifrSpec> {
        BiiW::new(self, 4)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    #[must_use]
    pub fn rxdr(&mut self) -> RxdrW<UartUrsifrSpec> {
        RxdrW::new(self, 5)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    #[must_use]
    pub fn txde(&mut self) -> TxdeW<UartUrsifrSpec> {
        TxdeW::new(self, 7)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<UartUrsifrSpec> {
        TxcW::new(self, 8)
    }
}
#[doc = "UART_URSIFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ursifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ursifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUrsifrSpec;
impl crate::RegisterSpec for UartUrsifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ursifr::R`](R) reader structure"]
impl crate::Readable for UartUrsifrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_ursifr::W`](W) writer structure"]
impl crate::Writable for UartUrsifrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_URSIFR to value 0"]
impl crate::Resettable for UartUrsifrSpec {
    const RESET_VALUE: u32 = 0;
}
