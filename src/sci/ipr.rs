#[doc = "Register `IPR` reader"]
pub type R = crate::R<IprSpec>;
#[doc = "Register `IPR` writer"]
pub type W = crate::W<IprSpec>;
#[doc = "Field `PARP` reader - PARP"]
pub type ParpR = crate::BitReader;
#[doc = "Field `PARP` writer - PARP"]
pub type ParpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCP` reader - RXCP"]
pub type RxcpR = crate::BitReader;
#[doc = "Field `RXCP` writer - RXCP"]
pub type RxcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCP` reader - TXCP"]
pub type TxcpR = crate::BitReader;
#[doc = "Field `TXCP` writer - TXCP"]
pub type TxcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTP` reader - WTP"]
pub type WtpR = crate::BitReader;
#[doc = "Field `WTP` writer - WTP"]
pub type WtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDIRP` reader - CARDIRP"]
pub type CardirpR = crate::BitReader;
#[doc = "Field `CARDIRP` writer - CARDIRP"]
pub type CardirpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEP` reader - TXBEP"]
pub type TxbepR = crate::BitReader;
#[doc = "Field `TXBEP` writer - TXBEP"]
pub type TxbepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    pub fn parp(&self) -> ParpR {
        ParpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    pub fn rxcp(&self) -> RxcpR {
        RxcpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    pub fn txcp(&self) -> TxcpR {
        TxcpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    pub fn wtp(&self) -> WtpR {
        WtpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    pub fn cardirp(&self) -> CardirpR {
        CardirpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    pub fn txbep(&self) -> TxbepR {
        TxbepR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    #[must_use]
    pub fn parp(&mut self) -> ParpW<IprSpec> {
        ParpW::new(self, 0)
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    #[must_use]
    pub fn rxcp(&mut self) -> RxcpW<IprSpec> {
        RxcpW::new(self, 1)
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    #[must_use]
    pub fn txcp(&mut self) -> TxcpW<IprSpec> {
        TxcpW::new(self, 2)
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    #[must_use]
    pub fn wtp(&mut self) -> WtpW<IprSpec> {
        WtpW::new(self, 3)
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    #[must_use]
    pub fn cardirp(&mut self) -> CardirpW<IprSpec> {
        CardirpW::new(self, 6)
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    #[must_use]
    pub fn txbep(&mut self) -> TxbepW<IprSpec> {
        TxbepW::new(self, 7)
    }
}
#[doc = "IPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IprSpec;
impl crate::RegisterSpec for IprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr::R`](R) reader structure"]
impl crate::Readable for IprSpec {}
#[doc = "`write(|w| ..)` method takes [`ipr::W`](W) writer structure"]
impl crate::Writable for IprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPR to value 0"]
impl crate::Resettable for IprSpec {
    const RESET_VALUE: u32 = 0;
}
