#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `PARE` reader - PARE"]
pub type PareR = crate::BitReader;
#[doc = "Field `PARE` writer - PARE"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCE` reader - RXCE"]
pub type RxceR = crate::BitReader;
#[doc = "Field `RXCE` writer - RXCE"]
pub type RxceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCE` reader - TXCE"]
pub type TxceR = crate::BitReader;
#[doc = "Field `TXCE` writer - TXCE"]
pub type TxceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTE` reader - WTE"]
pub type WteR = crate::BitReader;
#[doc = "Field `WTE` writer - WTE"]
pub type WteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDIRE` reader - CARDIRE"]
pub type CardireR = crate::BitReader;
#[doc = "Field `CARDIRE` writer - CARDIRE"]
pub type CardireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEE` reader - TXBEE"]
pub type TxbeeR = crate::BitReader;
#[doc = "Field `TXBEE` writer - TXBEE"]
pub type TxbeeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    pub fn rxce(&self) -> RxceR {
        RxceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    pub fn txce(&self) -> TxceR {
        TxceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    pub fn wte(&self) -> WteR {
        WteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    pub fn cardire(&self) -> CardireR {
        CardireR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    pub fn txbee(&self) -> TxbeeR {
        TxbeeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PareW<IerSpec> {
        PareW::new(self, 0)
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    #[must_use]
    pub fn rxce(&mut self) -> RxceW<IerSpec> {
        RxceW::new(self, 1)
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    #[must_use]
    pub fn txce(&mut self) -> TxceW<IerSpec> {
        TxceW::new(self, 2)
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WteW<IerSpec> {
        WteW::new(self, 3)
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    #[must_use]
    pub fn cardire(&mut self) -> CardireW<IerSpec> {
        CardireW::new(self, 6)
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    #[must_use]
    pub fn txbee(&mut self) -> TxbeeW<IerSpec> {
        TxbeeW::new(self, 7)
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
