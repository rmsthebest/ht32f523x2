#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXBEIEN` reader - TXBEIEN"]
pub type TxbeienR = crate::BitReader;
#[doc = "Field `TXBEIEN` writer - TXBEIEN"]
pub type TxbeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIEN` reader - TXEIEN"]
pub type TxeienR = crate::BitReader;
#[doc = "Field `TXEIEN` writer - TXEIEN"]
pub type TxeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBNEIEN` reader - RXBNEIEN"]
pub type RxbneienR = crate::BitReader;
#[doc = "Field `RXBNEIEN` writer - RXBNEIEN"]
pub type RxbneienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCIEN` reader - WCIEN"]
pub type WcienR = crate::BitReader;
#[doc = "Field `WCIEN` writer - WCIEN"]
pub type WcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIEN` reader - ROIEN"]
pub type RoienR = crate::BitReader;
#[doc = "Field `ROIEN` writer - ROIEN"]
pub type RoienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFIEN` reader - MFIEN"]
pub type MfienR = crate::BitReader;
#[doc = "Field `MFIEN` writer - MFIEN"]
pub type MfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAIEN` reader - SAIEN"]
pub type SaienR = crate::BitReader;
#[doc = "Field `SAIEN` writer - SAIEN"]
pub type SaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIEN` reader - TOIEN"]
pub type ToienR = crate::BitReader;
#[doc = "Field `TOIEN` writer - TOIEN"]
pub type ToienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    pub fn txbeien(&self) -> TxbeienR {
        TxbeienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    pub fn txeien(&self) -> TxeienR {
        TxeienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    pub fn rxbneien(&self) -> RxbneienR {
        RxbneienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    pub fn wcien(&self) -> WcienR {
        WcienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    pub fn roien(&self) -> RoienR {
        RoienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    pub fn mfien(&self) -> MfienR {
        MfienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    pub fn saien(&self) -> SaienR {
        SaienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    pub fn toien(&self) -> ToienR {
        ToienR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txbeien(&mut self) -> TxbeienW<IerSpec> {
        TxbeienW::new(self, 0)
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txeien(&mut self) -> TxeienW<IerSpec> {
        TxeienW::new(self, 1)
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxbneien(&mut self) -> RxbneienW<IerSpec> {
        RxbneienW::new(self, 2)
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wcien(&mut self) -> WcienW<IerSpec> {
        WcienW::new(self, 3)
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    #[must_use]
    pub fn roien(&mut self) -> RoienW<IerSpec> {
        RoienW::new(self, 4)
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn mfien(&mut self) -> MfienW<IerSpec> {
        MfienW::new(self, 5)
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SaienW<IerSpec> {
        SaienW::new(self, 6)
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    #[must_use]
    pub fn toien(&mut self) -> ToienW<IerSpec> {
        ToienW::new(self, 7)
    }
}
#[doc = "IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
