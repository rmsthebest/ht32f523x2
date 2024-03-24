#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `I2SEN` reader - I2SEN"]
pub type I2senR = crate::BitReader;
#[doc = "Field `I2SEN` writer - I2SEN"]
pub type I2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - TXEN"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - TXEN"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - RXEN"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - RXEN"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSIZE` reader - SMPSIZE"]
pub type SmpsizeR = crate::FieldReader;
#[doc = "Field `SMPSIZE` writer - SMPSIZE"]
pub type SmpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORMAT` reader - FORMAT"]
pub type FormatR = crate::FieldReader;
#[doc = "Field `FORMAT` writer - FORMAT"]
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BITEXT` reader - BITEXT"]
pub type BitextR = crate::BitReader;
#[doc = "Field `BITEXT` writer - BITEXT"]
pub type BitextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKEN` reader - MCLKEN"]
pub type MclkenR = crate::BitReader;
#[doc = "Field `MCLKEN` writer - MCLKEN"]
pub type MclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPEAT` reader - REPEAT"]
pub type RepeatR = crate::BitReader;
#[doc = "Field `REPEAT` writer - REPEAT"]
pub type RepeatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL` reader - CHANNEL"]
pub type ChannelR = crate::BitReader;
#[doc = "Field `CHANNEL` writer - CHANNEL"]
pub type ChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMUTE` reader - TXMUTE"]
pub type TxmuteR = crate::BitReader;
#[doc = "Field `TXMUTE` writer - TXMUTE"]
pub type TxmuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDEN` reader - CLKDEN"]
pub type ClkdenR = crate::BitReader;
#[doc = "Field `CLKDEN` writer - CLKDEN"]
pub type ClkdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCEN` reader - RCEN"]
pub type RcenR = crate::BitReader;
#[doc = "Field `RCEN` writer - RCEN"]
pub type RcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCSEL` reader - RCSEL"]
pub type RcselR = crate::BitReader;
#[doc = "Field `RCSEL` writer - RCSEL"]
pub type RcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCKINV` reader - BCKINV"]
pub type BckinvR = crate::BitReader;
#[doc = "Field `BCKINV` writer - BCKINV"]
pub type BckinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKINV` reader - MCKINV"]
pub type MckinvR = crate::BitReader;
#[doc = "Field `MCKINV` writer - MCKINV"]
pub type MckinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2senR {
        I2senR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    pub fn smpsize(&self) -> SmpsizeR {
        SmpsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    pub fn bitext(&self) -> BitextR {
        BitextR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    pub fn mclken(&self) -> MclkenR {
        MclkenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    pub fn repeat(&self) -> RepeatR {
        RepeatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    pub fn txmute(&self) -> TxmuteR {
        TxmuteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    pub fn clkden(&self) -> ClkdenR {
        ClkdenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    pub fn rcen(&self) -> RcenR {
        RcenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    pub fn rcsel(&self) -> RcselR {
        RcselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    pub fn bckinv(&self) -> BckinvR {
        BckinvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    pub fn mckinv(&self) -> MckinvR {
        MckinvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2senW<CrSpec> {
        I2senW::new(self, 0)
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<CrSpec> {
        TxenW::new(self, 1)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CrSpec> {
        RxenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn smpsize(&mut self) -> SmpsizeW<CrSpec> {
        SmpsizeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FormatW<CrSpec> {
        FormatW::new(self, 6)
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    #[must_use]
    pub fn bitext(&mut self) -> BitextW<CrSpec> {
        BitextW::new(self, 8)
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mclken(&mut self) -> MclkenW<CrSpec> {
        MclkenW::new(self, 9)
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> RepeatW<CrSpec> {
        RepeatW::new(self, 10)
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> ChannelW<CrSpec> {
        ChannelW::new(self, 11)
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    #[must_use]
    pub fn txmute(&mut self) -> TxmuteW<CrSpec> {
        TxmuteW::new(self, 12)
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<CrSpec> {
        TxdmaenW::new(self, 13)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<CrSpec> {
        RxdmaenW::new(self, 14)
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    #[must_use]
    pub fn clkden(&mut self) -> ClkdenW<CrSpec> {
        ClkdenW::new(self, 15)
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    #[must_use]
    pub fn rcen(&mut self) -> RcenW<CrSpec> {
        RcenW::new(self, 16)
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rcsel(&mut self) -> RcselW<CrSpec> {
        RcselW::new(self, 17)
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    #[must_use]
    pub fn bckinv(&mut self) -> BckinvW<CrSpec> {
        BckinvW::new(self, 18)
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    #[must_use]
    pub fn mckinv(&mut self) -> MckinvW<CrSpec> {
        MckinvW::new(self, 19)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
