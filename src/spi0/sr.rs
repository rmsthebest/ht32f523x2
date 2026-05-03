#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `TXBE` reader - TXBE"]
pub type TxbeR = crate::BitReader;
#[doc = "Field `TXBE` writer - TXBE"]
pub type TxbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBNE` reader - RXBNE"]
pub type RxbneR = crate::BitReader;
#[doc = "Field `RXBNE` writer - RXBNE"]
pub type RxbneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WC` reader - WC"]
pub type WcR = crate::BitReader;
#[doc = "Field `WC` writer - WC"]
pub type WcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RO` reader - RO"]
pub type RoR = crate::BitReader;
#[doc = "Field `RO` writer - RO"]
pub type RoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MF` reader - MF"]
pub type MfR = crate::BitReader;
#[doc = "Field `MF` writer - MF"]
pub type MfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA` reader - SA"]
pub type SaR = crate::BitReader;
#[doc = "Field `SA` writer - SA"]
pub type SaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO` reader - TO"]
pub type ToR = crate::BitReader;
#[doc = "Field `TO` writer - TO"]
pub type ToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    pub fn txbe(&self) -> TxbeR {
        TxbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    pub fn rxbne(&self) -> RxbneR {
        RxbneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    pub fn wc(&self) -> WcR {
        WcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    pub fn mf(&self) -> MfR {
        MfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    pub fn txbe(&mut self) -> TxbeW<'_, SrSpec> {
        TxbeW::new(self, 0)
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, SrSpec> {
        TxeW::new(self, 1)
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    pub fn rxbne(&mut self) -> RxbneW<'_, SrSpec> {
        RxbneW::new(self, 2)
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    pub fn wc(&mut self) -> WcW<'_, SrSpec> {
        WcW::new(self, 3)
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    pub fn ro(&mut self) -> RoW<'_, SrSpec> {
        RoW::new(self, 4)
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    pub fn mf(&mut self) -> MfW<'_, SrSpec> {
        MfW::new(self, 5)
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<'_, SrSpec> {
        SaW::new(self, 6)
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<'_, SrSpec> {
        ToW::new(self, 7)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, SrSpec> {
        BusyW::new(self, 8)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
