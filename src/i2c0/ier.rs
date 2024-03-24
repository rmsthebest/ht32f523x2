#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `STAIE` reader - STAIE"]
pub type StaieR = crate::BitReader;
#[doc = "Field `STAIE` writer - STAIE"]
pub type StaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOIE` reader - STOIE"]
pub type StoieR = crate::BitReader;
#[doc = "Field `STOIE` writer - STOIE"]
pub type StoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRSIE` reader - ADRSIE"]
pub type AdrsieR = crate::BitReader;
#[doc = "Field `ADRSIE` writer - ADRSIE"]
pub type AdrsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCSIE` reader - GCSIE"]
pub type GcsieR = crate::BitReader;
#[doc = "Field `GCSIE` writer - GCSIE"]
pub type GcsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOSIE` reader - ARBLOSIE"]
pub type ArblosieR = crate::BitReader;
#[doc = "Field `ARBLOSIE` writer - ARBLOSIE"]
pub type ArblosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNACKIE` reader - RXNACKIE"]
pub type RxnackieR = crate::BitReader;
#[doc = "Field `RXNACKIE` writer - RXNACKIE"]
pub type RxnackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRIE` reader - BUSERRIE"]
pub type BuserrieR = crate::BitReader;
#[doc = "Field `BUSERRIE` writer - BUSERRIE"]
pub type BuserrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUTIE` reader - TOUTIE"]
pub type ToutieR = crate::BitReader;
#[doc = "Field `TOUTIE` writer - TOUTIE"]
pub type ToutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDNEIE` reader - RXDNEIE"]
pub type RxdneieR = crate::BitReader;
#[doc = "Field `RXDNEIE` writer - RXDNEIE"]
pub type RxdneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDEIE` reader - TXDEIE"]
pub type TxdeieR = crate::BitReader;
#[doc = "Field `TXDEIE` writer - TXDEIE"]
pub type TxdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBFIE` reader - RXBFIE"]
pub type RxbfieR = crate::BitReader;
#[doc = "Field `RXBFIE` writer - RXBFIE"]
pub type RxbfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    pub fn staie(&self) -> StaieR {
        StaieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    pub fn stoie(&self) -> StoieR {
        StoieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    pub fn adrsie(&self) -> AdrsieR {
        AdrsieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    pub fn gcsie(&self) -> GcsieR {
        GcsieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    pub fn arblosie(&self) -> ArblosieR {
        ArblosieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    pub fn rxnackie(&self) -> RxnackieR {
        RxnackieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    pub fn buserrie(&self) -> BuserrieR {
        BuserrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    pub fn toutie(&self) -> ToutieR {
        ToutieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    pub fn rxdneie(&self) -> RxdneieR {
        RxdneieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TxdeieR {
        TxdeieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RxbfieR {
        RxbfieR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    #[must_use]
    pub fn staie(&mut self) -> StaieW<IerSpec> {
        StaieW::new(self, 0)
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    #[must_use]
    pub fn stoie(&mut self) -> StoieW<IerSpec> {
        StoieW::new(self, 1)
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    #[must_use]
    pub fn adrsie(&mut self) -> AdrsieW<IerSpec> {
        AdrsieW::new(self, 2)
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    #[must_use]
    pub fn gcsie(&mut self) -> GcsieW<IerSpec> {
        GcsieW::new(self, 3)
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    #[must_use]
    pub fn arblosie(&mut self) -> ArblosieW<IerSpec> {
        ArblosieW::new(self, 8)
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxnackie(&mut self) -> RxnackieW<IerSpec> {
        RxnackieW::new(self, 9)
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn buserrie(&mut self) -> BuserrieW<IerSpec> {
        BuserrieW::new(self, 10)
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn toutie(&mut self) -> ToutieW<IerSpec> {
        ToutieW::new(self, 11)
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdneie(&mut self) -> RxdneieW<IerSpec> {
        RxdneieW::new(self, 16)
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TxdeieW<IerSpec> {
        TxdeieW::new(self, 17)
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbfie(&mut self) -> RxbfieW<IerSpec> {
        RxbfieW::new(self, 18)
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
