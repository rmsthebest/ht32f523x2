#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXFTLIEN` reader - TXFTLIEN"]
pub type TxftlienR = crate::BitReader;
#[doc = "Field `TXFTLIEN` writer - TXFTLIEN"]
pub type TxftlienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUDIEN` reader - TXUDIEN"]
pub type TxudienR = crate::BitReader;
#[doc = "Field `TXUDIEN` writer - TXUDIEN"]
pub type TxudienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOVIEN` reader - TXOVIEN"]
pub type TxovienR = crate::BitReader;
#[doc = "Field `TXOVIEN` writer - TXOVIEN"]
pub type TxovienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTLIEN` reader - RXFTLIEN"]
pub type RxftlienR = crate::BitReader;
#[doc = "Field `RXFTLIEN` writer - RXFTLIEN"]
pub type RxftlienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDIEN` reader - RXUDIEN"]
pub type RxudienR = crate::BitReader;
#[doc = "Field `RXUDIEN` writer - RXUDIEN"]
pub type RxudienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVIEN` reader - RXOVIEN"]
pub type RxovienR = crate::BitReader;
#[doc = "Field `RXOVIEN` writer - RXOVIEN"]
pub type RxovienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    pub fn txftlien(&self) -> TxftlienR {
        TxftlienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    pub fn txudien(&self) -> TxudienR {
        TxudienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    pub fn txovien(&self) -> TxovienR {
        TxovienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    pub fn rxftlien(&self) -> RxftlienR {
        RxftlienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    pub fn rxudien(&self) -> RxudienR {
        RxudienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    pub fn rxovien(&self) -> RxovienR {
        RxovienR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    pub fn txftlien(&mut self) -> TxftlienW<'_, IerSpec> {
        TxftlienW::new(self, 0)
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    pub fn txudien(&mut self) -> TxudienW<'_, IerSpec> {
        TxudienW::new(self, 1)
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    pub fn txovien(&mut self) -> TxovienW<'_, IerSpec> {
        TxovienW::new(self, 2)
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    pub fn rxftlien(&mut self) -> RxftlienW<'_, IerSpec> {
        RxftlienW::new(self, 4)
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    pub fn rxudien(&mut self) -> RxudienW<'_, IerSpec> {
        RxudienW::new(self, 5)
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    pub fn rxovien(&mut self) -> RxovienW<'_, IerSpec> {
        RxovienW::new(self, 6)
    }
}
#[doc = "IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
