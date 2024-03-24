#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `AA` reader - AA"]
pub type AaR = crate::BitReader;
#[doc = "Field `AA` writer - AA"]
pub type AaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN` reader - GCEN"]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - GCEN"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - I2CEN"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2CEN"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRM` reader - ADRM"]
pub type AdrmR = crate::BitReader;
#[doc = "Field `ADRM` writer - ADRM"]
pub type AdrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMANACK` reader - DMANACK"]
pub type DmanackR = crate::BitReader;
#[doc = "Field `DMANACK` writer - DMANACK"]
pub type DmanackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTOUT` reader - ENTOUT"]
pub type EntoutR = crate::BitReader;
#[doc = "Field `ENTOUT` writer - ENTOUT"]
pub type EntoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMB_FILTER_EN` reader - COMB_FILTER_EN"]
pub type CombFilterEnR = crate::BitReader;
#[doc = "Field `COMB_FILTER_EN` writer - COMB_FILTER_EN"]
pub type CombFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_FILTER` reader - SEQ_FILTER"]
pub type SeqFilterR = crate::FieldReader;
#[doc = "Field `SEQ_FILTER` writer - SEQ_FILTER"]
pub type SeqFilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    pub fn aa(&self) -> AaR {
        AaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    pub fn adrm(&self) -> AdrmR {
        AdrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMANACK"]
    #[inline(always)]
    pub fn dmanack(&self) -> DmanackR {
        DmanackR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    pub fn entout(&self) -> EntoutR {
        EntoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMB_FILTER_EN"]
    #[inline(always)]
    pub fn comb_filter_en(&self) -> CombFilterEnR {
        CombFilterEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - SEQ_FILTER"]
    #[inline(always)]
    pub fn seq_filter(&self) -> SeqFilterR {
        SeqFilterR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AaW<CrSpec> {
        AaW::new(self, 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CrSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GcenW<CrSpec> {
        GcenW::new(self, 2)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<CrSpec> {
        I2cenW::new(self, 3)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    #[must_use]
    pub fn adrm(&mut self) -> AdrmW<CrSpec> {
        AdrmW::new(self, 7)
    }
    #[doc = "Bit 8 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<CrSpec> {
        TxdmaeW::new(self, 8)
    }
    #[doc = "Bit 9 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<CrSpec> {
        RxdmaeW::new(self, 9)
    }
    #[doc = "Bit 10 - DMANACK"]
    #[inline(always)]
    #[must_use]
    pub fn dmanack(&mut self) -> DmanackW<CrSpec> {
        DmanackW::new(self, 10)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    #[must_use]
    pub fn entout(&mut self) -> EntoutW<CrSpec> {
        EntoutW::new(self, 12)
    }
    #[doc = "Bit 13 - COMB_FILTER_EN"]
    #[inline(always)]
    #[must_use]
    pub fn comb_filter_en(&mut self) -> CombFilterEnW<CrSpec> {
        CombFilterEnW::new(self, 13)
    }
    #[doc = "Bits 14:15 - SEQ_FILTER"]
    #[inline(always)]
    #[must_use]
    pub fn seq_filter(&mut self) -> SeqFilterW<CrSpec> {
        SeqFilterW::new(self, 14)
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
