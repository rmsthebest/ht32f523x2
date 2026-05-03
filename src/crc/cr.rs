#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `POLY` reader - POLY"]
pub type PolyR = crate::FieldReader;
#[doc = "Field `POLY` writer - POLY"]
pub type PolyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATBIRV` reader - DATBIRV"]
pub type DatbirvR = crate::BitReader;
#[doc = "Field `DATBIRV` writer - DATBIRV"]
pub type DatbirvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATBYRV` reader - DATBYRV"]
pub type DatbyrvR = crate::BitReader;
#[doc = "Field `DATBYRV` writer - DATBYRV"]
pub type DatbyrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCMPL` reader - DATCMPL"]
pub type DatcmplR = crate::BitReader;
#[doc = "Field `DATCMPL` writer - DATCMPL"]
pub type DatcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUMBIRV` reader - SUMBIRV"]
pub type SumbirvR = crate::BitReader;
#[doc = "Field `SUMBIRV` writer - SUMBIRV"]
pub type SumbirvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUMBYRV` reader - SUMBYRV"]
pub type SumbyrvR = crate::BitReader;
#[doc = "Field `SUMBYRV` writer - SUMBYRV"]
pub type SumbyrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUMCMPL` reader - SUMCMPL"]
pub type SumcmplR = crate::BitReader;
#[doc = "Field `SUMCMPL` writer - SUMCMPL"]
pub type SumcmplW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    pub fn poly(&self) -> PolyR {
        PolyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    pub fn datbirv(&self) -> DatbirvR {
        DatbirvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    pub fn datbyrv(&self) -> DatbyrvR {
        DatbyrvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    pub fn datcmpl(&self) -> DatcmplR {
        DatcmplR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    pub fn sumbirv(&self) -> SumbirvR {
        SumbirvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    pub fn sumbyrv(&self) -> SumbyrvR {
        SumbyrvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    pub fn sumcmpl(&self) -> SumcmplR {
        SumcmplR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    pub fn poly(&mut self) -> PolyW<'_, CrSpec> {
        PolyW::new(self, 0)
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    pub fn datbirv(&mut self) -> DatbirvW<'_, CrSpec> {
        DatbirvW::new(self, 2)
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    pub fn datbyrv(&mut self) -> DatbyrvW<'_, CrSpec> {
        DatbyrvW::new(self, 3)
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    pub fn datcmpl(&mut self) -> DatcmplW<'_, CrSpec> {
        DatcmplW::new(self, 4)
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    pub fn sumbirv(&mut self) -> SumbirvW<'_, CrSpec> {
        SumbirvW::new(self, 5)
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    pub fn sumbyrv(&mut self) -> SumbyrvW<'_, CrSpec> {
        SumbyrvW::new(self, 6)
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    pub fn sumcmpl(&mut self) -> SumcmplW<'_, CrSpec> {
        SumcmplW::new(self, 7)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
