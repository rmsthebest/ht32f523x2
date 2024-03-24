#[doc = "Register `UART_URIER` reader"]
pub type R = crate::R<UartUrierSpec>;
#[doc = "Register `UART_URIER` writer"]
pub type W = crate::W<UartUrierSpec>;
#[doc = "Field `RXDRIE` reader - RXDRIE"]
pub type RxdrieR = crate::BitReader;
#[doc = "Field `RXDRIE` writer - RXDRIE"]
pub type RxdrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDEIE` reader - TXDEIE"]
pub type TxdeieR = crate::BitReader;
#[doc = "Field `TXDEIE` writer - TXDEIE"]
pub type TxdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIE` reader - TXCIE"]
pub type TxcieR = crate::BitReader;
#[doc = "Field `TXCIE` writer - TXCIE"]
pub type TxcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIE` reader - OEIE"]
pub type OeieR = crate::BitReader;
#[doc = "Field `OEIE` writer - OEIE"]
pub type OeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PEIE"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PEIE"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIE` reader - FEIE"]
pub type FeieR = crate::BitReader;
#[doc = "Field `FEIE` writer - FEIE"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - BIE"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - BIE"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RxdrieR {
        RxdrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TxdeieR {
        TxdeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&self) -> TxcieR {
        TxcieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    pub fn oeie(&self) -> OeieR {
        OeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrie(&mut self) -> RxdrieW<UartUrierSpec> {
        RxdrieW::new(self, 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TxdeieW<UartUrierSpec> {
        TxdeieW::new(self, 1)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TxcieW<UartUrierSpec> {
        TxcieW::new(self, 2)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    #[must_use]
    pub fn oeie(&mut self) -> OeieW<UartUrierSpec> {
        OeieW::new(self, 3)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<UartUrierSpec> {
        PeieW::new(self, 4)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<UartUrierSpec> {
        FeieW::new(self, 5)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BieW<UartUrierSpec> {
        BieW::new(self, 6)
    }
}
#[doc = "UART_URIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUrierSpec;
impl crate::RegisterSpec for UartUrierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_urier::R`](R) reader structure"]
impl crate::Readable for UartUrierSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_urier::W`](W) writer structure"]
impl crate::Writable for UartUrierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_URIER to value 0"]
impl crate::Resettable for UartUrierSpec {
    const RESET_VALUE: u32 = 0;
}
