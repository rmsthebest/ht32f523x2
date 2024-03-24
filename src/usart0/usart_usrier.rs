#[doc = "Register `USART_USRIER` reader"]
pub type R = crate::R<UsartUsrierSpec>;
#[doc = "Register `USART_USRIER` writer"]
pub type W = crate::W<UsartUsrierSpec>;
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
#[doc = "Field `RSADDIE` reader - RSADDIE"]
pub type RsaddieR = crate::BitReader;
#[doc = "Field `RSADDIE` writer - RSADDIE"]
pub type RsaddieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTOIE` reader - RXTOIE"]
pub type RxtoieR = crate::BitReader;
#[doc = "Field `RXTOIE` writer - RXTOIE"]
pub type RxtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTSIE"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTSIE"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - RSADDIE"]
    #[inline(always)]
    pub fn rsaddie(&self) -> RsaddieR {
        RsaddieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RXTOIE"]
    #[inline(always)]
    pub fn rxtoie(&self) -> RxtoieR {
        RxtoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSIE"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrie(&mut self) -> RxdrieW<UsartUsrierSpec> {
        RxdrieW::new(self, 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TxdeieW<UsartUsrierSpec> {
        TxdeieW::new(self, 1)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TxcieW<UsartUsrierSpec> {
        TxcieW::new(self, 2)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    #[must_use]
    pub fn oeie(&mut self) -> OeieW<UsartUsrierSpec> {
        OeieW::new(self, 3)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<UsartUsrierSpec> {
        PeieW::new(self, 4)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<UsartUsrierSpec> {
        FeieW::new(self, 5)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BieW<UsartUsrierSpec> {
        BieW::new(self, 6)
    }
    #[doc = "Bit 7 - RSADDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rsaddie(&mut self) -> RsaddieW<UsartUsrierSpec> {
        RsaddieW::new(self, 7)
    }
    #[doc = "Bit 8 - RXTOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoie(&mut self) -> RxtoieW<UsartUsrierSpec> {
        RxtoieW::new(self, 8)
    }
    #[doc = "Bit 9 - CTSIE"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CtsieW<UsartUsrierSpec> {
        CtsieW::new(self, 9)
    }
}
#[doc = "USART_USRIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrierSpec;
impl crate::RegisterSpec for UsartUsrierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrier::R`](R) reader structure"]
impl crate::Readable for UsartUsrierSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrier::W`](W) writer structure"]
impl crate::Writable for UsartUsrierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRIER to value 0"]
impl crate::Resettable for UsartUsrierSpec {
    const RESET_VALUE: u32 = 0;
}
