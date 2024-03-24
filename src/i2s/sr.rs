#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `TXFTL` reader - TXFTL"]
pub type TxftlR = crate::BitReader;
#[doc = "Field `TXFTL` writer - TXFTL"]
pub type TxftlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFUD` reader - TXFUD"]
pub type TxfudR = crate::BitReader;
#[doc = "Field `TXFUD` writer - TXFUD"]
pub type TxfudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFOV` reader - TXFOV"]
pub type TxfovR = crate::BitReader;
#[doc = "Field `TXFOV` writer - TXFOV"]
pub type TxfovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMT` reader - TXFEMT"]
pub type TxfemtR = crate::BitReader;
#[doc = "Field `TXFEMT` writer - TXFEMT"]
pub type TxfemtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFUL` reader - TXFFUL"]
pub type TxffulR = crate::BitReader;
#[doc = "Field `TXFFUL` writer - TXFFUL"]
pub type TxffulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTL` reader - RXFTL"]
pub type RxftlR = crate::BitReader;
#[doc = "Field `RXFTL` writer - RXFTL"]
pub type RxftlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFUD` reader - RXFUD"]
pub type RxfudR = crate::BitReader;
#[doc = "Field `RXFUD` writer - RXFUD"]
pub type RxfudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFOV` reader - RXFOV"]
pub type RxfovR = crate::BitReader;
#[doc = "Field `RXFOV` writer - RXFOV"]
pub type RxfovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFEMT` reader - RXFEMT"]
pub type RxfemtR = crate::BitReader;
#[doc = "Field `RXFEMT` writer - RXFEMT"]
pub type RxfemtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFUL` reader - RXFFUL"]
pub type RxffulR = crate::BitReader;
#[doc = "Field `RXFFUL` writer - RXFFUL"]
pub type RxffulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHS` reader - CHS"]
pub type ChsR = crate::BitReader;
#[doc = "Field `CHS` writer - CHS"]
pub type ChsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUSY` reader - TXBUSY"]
pub type TxbusyR = crate::BitReader;
#[doc = "Field `TXBUSY` writer - TXBUSY"]
pub type TxbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY` reader - CLKRDY"]
pub type ClkrdyR = crate::BitReader;
#[doc = "Field `CLKRDY` writer - CLKRDY"]
pub type ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFS` reader - TXFS"]
pub type TxfsR = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RxfsR = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    pub fn txftl(&self) -> TxftlR {
        TxftlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    pub fn txfud(&self) -> TxfudR {
        TxfudR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    pub fn txfov(&self) -> TxfovR {
        TxfovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    pub fn txfemt(&self) -> TxfemtR {
        TxfemtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    pub fn txfful(&self) -> TxffulR {
        TxffulR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    pub fn rxftl(&self) -> RxftlR {
        RxftlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    pub fn rxfud(&self) -> RxfudR {
        RxfudR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    pub fn rxfov(&self) -> RxfovR {
        RxfovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    pub fn rxfemt(&self) -> RxfemtR {
        RxfemtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    pub fn rxfful(&self) -> RxffulR {
        RxffulR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    pub fn chs(&self) -> ChsR {
        ChsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TxfsR {
        TxfsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RxfsR {
        RxfsR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    #[must_use]
    pub fn txftl(&mut self) -> TxftlW<SrSpec> {
        TxftlW::new(self, 0)
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    #[must_use]
    pub fn txfud(&mut self) -> TxfudW<SrSpec> {
        TxfudW::new(self, 1)
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    #[must_use]
    pub fn txfov(&mut self) -> TxfovW<SrSpec> {
        TxfovW::new(self, 2)
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    #[must_use]
    pub fn txfemt(&mut self) -> TxfemtW<SrSpec> {
        TxfemtW::new(self, 3)
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    #[must_use]
    pub fn txfful(&mut self) -> TxffulW<SrSpec> {
        TxffulW::new(self, 4)
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    #[must_use]
    pub fn rxftl(&mut self) -> RxftlW<SrSpec> {
        RxftlW::new(self, 8)
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    #[must_use]
    pub fn rxfud(&mut self) -> RxfudW<SrSpec> {
        RxfudW::new(self, 9)
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    #[must_use]
    pub fn rxfov(&mut self) -> RxfovW<SrSpec> {
        RxfovW::new(self, 10)
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    #[must_use]
    pub fn rxfemt(&mut self) -> RxfemtW<SrSpec> {
        RxfemtW::new(self, 11)
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    #[must_use]
    pub fn rxfful(&mut self) -> RxffulW<SrSpec> {
        RxffulW::new(self, 12)
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    #[must_use]
    pub fn chs(&mut self) -> ChsW<SrSpec> {
        ChsW::new(self, 16)
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn txbusy(&mut self) -> TxbusyW<SrSpec> {
        TxbusyW::new(self, 17)
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> ClkrdyW<SrSpec> {
        ClkrdyW::new(self, 18)
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    #[must_use]
    pub fn txfs(&mut self) -> TxfsW<SrSpec> {
        TxfsW::new(self, 24)
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfs(&mut self) -> RxfsW<SrSpec> {
        RxfsW::new(self, 28)
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
