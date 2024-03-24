#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `STA` reader - STA"]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - STA"]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STO` reader - STO"]
pub type StoR = crate::BitReader;
#[doc = "Field `STO` writer - STO"]
pub type StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRS` reader - ADRS"]
pub type AdrsR = crate::BitReader;
#[doc = "Field `ADRS` writer - ADRS"]
pub type AdrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCS` reader - GCS"]
pub type GcsR = crate::BitReader;
#[doc = "Field `GCS` writer - GCS"]
pub type GcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOS` reader - ARBLOS"]
pub type ArblosR = crate::BitReader;
#[doc = "Field `ARBLOS` writer - ARBLOS"]
pub type ArblosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNACK` reader - RXNACK"]
pub type RxnackR = crate::BitReader;
#[doc = "Field `RXNACK` writer - RXNACK"]
pub type RxnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` reader - BUSERR"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - BUSERR"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUTF` reader - TOUTF"]
pub type ToutfR = crate::BitReader;
#[doc = "Field `TOUTF` writer - TOUTF"]
pub type ToutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDNE` reader - RXDNE"]
pub type RxdneR = crate::BitReader;
#[doc = "Field `RXDNE` writer - RXDNE"]
pub type RxdneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TxdeR = crate::BitReader;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TxdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBF` reader - RXBF"]
pub type RxbfR = crate::BitReader;
#[doc = "Field `RXBF` writer - RXBF"]
pub type RxbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSBUSY` reader - BUSBUSY"]
pub type BusbusyR = crate::BitReader;
#[doc = "Field `BUSBUSY` writer - BUSBUSY"]
pub type BusbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER` reader - MASTER"]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - MASTER"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNRX` reader - TXNRX"]
pub type TxnrxR = crate::BitReader;
#[doc = "Field `TXNRX` writer - TXNRX"]
pub type TxnrxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    pub fn sto(&self) -> StoR {
        StoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    pub fn adrs(&self) -> AdrsR {
        AdrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GcsR {
        GcsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    pub fn arblos(&self) -> ArblosR {
        ArblosR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    pub fn rxnack(&self) -> RxnackR {
        RxnackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    pub fn toutf(&self) -> ToutfR {
        ToutfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RxdneR {
        RxdneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TxdeR {
        TxdeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    pub fn rxbf(&self) -> RxbfR {
        RxbfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&self) -> BusbusyR {
        BusbusyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    pub fn txnrx(&self) -> TxnrxR {
        TxnrxR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<SrSpec> {
        StaW::new(self, 0)
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> StoW<SrSpec> {
        StoW::new(self, 1)
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    #[must_use]
    pub fn adrs(&mut self) -> AdrsW<SrSpec> {
        AdrsW::new(self, 2)
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GcsW<SrSpec> {
        GcsW::new(self, 3)
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    #[must_use]
    pub fn arblos(&mut self) -> ArblosW<SrSpec> {
        ArblosW::new(self, 8)
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    #[must_use]
    pub fn rxnack(&mut self) -> RxnackW<SrSpec> {
        RxnackW::new(self, 9)
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BuserrW<SrSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    #[must_use]
    pub fn toutf(&mut self) -> ToutfW<SrSpec> {
        ToutfW::new(self, 11)
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdne(&mut self) -> RxdneW<SrSpec> {
        RxdneW::new(self, 16)
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    #[must_use]
    pub fn txde(&mut self) -> TxdeW<SrSpec> {
        TxdeW::new(self, 17)
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    #[must_use]
    pub fn rxbf(&mut self) -> RxbfW<SrSpec> {
        RxbfW::new(self, 18)
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busbusy(&mut self) -> BusbusyW<SrSpec> {
        BusbusyW::new(self, 19)
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<SrSpec> {
        MasterW::new(self, 20)
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    #[must_use]
    pub fn txnrx(&mut self) -> TxnrxW<SrSpec> {
        TxnrxW::new(self, 21)
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
