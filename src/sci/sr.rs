#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `PARF` reader - PARF"]
pub type ParfR = crate::BitReader;
#[doc = "Field `PARF` writer - PARF"]
pub type ParfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCF` reader - RXCF"]
pub type RxcfR = crate::BitReader;
#[doc = "Field `RXCF` writer - RXCF"]
pub type RxcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCF` reader - TXCF"]
pub type TxcfR = crate::BitReader;
#[doc = "Field `TXCF` writer - TXCF"]
pub type TxcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTF` reader - WTF"]
pub type WtfR = crate::BitReader;
#[doc = "Field `WTF` writer - WTF"]
pub type WtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPREF` reader - CPREF"]
pub type CprefR = crate::BitReader;
#[doc = "Field `CPREF` writer - CPREF"]
pub type CprefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEF` reader - TXBEF"]
pub type TxbefR = crate::BitReader;
#[doc = "Field `TXBEF` writer - TXBEF"]
pub type TxbefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    pub fn parf(&self) -> ParfR {
        ParfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    pub fn rxcf(&self) -> RxcfR {
        RxcfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    pub fn txcf(&self) -> TxcfR {
        TxcfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    pub fn wtf(&self) -> WtfR {
        WtfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    pub fn cpref(&self) -> CprefR {
        CprefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    pub fn txbef(&self) -> TxbefR {
        TxbefR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    #[must_use]
    pub fn parf(&mut self) -> ParfW<SrSpec> {
        ParfW::new(self, 0)
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxcf(&mut self) -> RxcfW<SrSpec> {
        RxcfW::new(self, 1)
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    #[must_use]
    pub fn txcf(&mut self) -> TxcfW<SrSpec> {
        TxcfW::new(self, 2)
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    #[must_use]
    pub fn wtf(&mut self) -> WtfW<SrSpec> {
        WtfW::new(self, 3)
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    #[must_use]
    pub fn cpref(&mut self) -> CprefW<SrSpec> {
        CprefW::new(self, 6)
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    #[must_use]
    pub fn txbef(&mut self) -> TxbefW<SrSpec> {
        TxbefW::new(self, 7)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
