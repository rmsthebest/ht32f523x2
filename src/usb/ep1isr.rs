#[doc = "Register `EP1ISR` reader"]
pub type R = crate::R<Ep1isrSpec>;
#[doc = "Register `EP1ISR` writer"]
pub type W = crate::W<Ep1isrSpec>;
#[doc = "Field `OTRXIF` reader - OTRXIF"]
pub type OtrxifR = crate::BitReader;
#[doc = "Field `OTRXIF` writer - OTRXIF"]
pub type OtrxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODRXIF` reader - ODRXIF"]
pub type OdrxifR = crate::BitReader;
#[doc = "Field `ODRXIF` writer - ODRXIF"]
pub type OdrxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODOVIF` reader - ODOVIF"]
pub type OdovifR = crate::BitReader;
#[doc = "Field `ODOVIF` writer - ODOVIF"]
pub type OdovifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITRXIF` reader - ITRXIF"]
pub type ItrxifR = crate::BitReader;
#[doc = "Field `ITRXIF` writer - ITRXIF"]
pub type ItrxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDTXIF` reader - IDTXIF"]
pub type IdtxifR = crate::BitReader;
#[doc = "Field `IDTXIF` writer - IDTXIF"]
pub type IdtxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKIF` reader - NAKIF"]
pub type NakifR = crate::BitReader;
#[doc = "Field `NAKIF` writer - NAKIF"]
pub type NakifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STLIF` reader - STLIF"]
pub type StlifR = crate::BitReader;
#[doc = "Field `STLIF` writer - STLIF"]
pub type StlifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UERIF` reader - UERIF"]
pub type UerifR = crate::BitReader;
#[doc = "Field `UERIF` writer - UERIF"]
pub type UerifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    pub fn otrxif(&self) -> OtrxifR {
        OtrxifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    pub fn odrxif(&self) -> OdrxifR {
        OdrxifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    pub fn odovif(&self) -> OdovifR {
        OdovifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    pub fn itrxif(&self) -> ItrxifR {
        ItrxifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    pub fn idtxif(&self) -> IdtxifR {
        IdtxifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    pub fn nakif(&self) -> NakifR {
        NakifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    pub fn stlif(&self) -> StlifR {
        StlifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    pub fn uerif(&self) -> UerifR {
        UerifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn otrxif(&mut self) -> OtrxifW<Ep1isrSpec> {
        OtrxifW::new(self, 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn odrxif(&mut self) -> OdrxifW<Ep1isrSpec> {
        OdrxifW::new(self, 1)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    #[must_use]
    pub fn odovif(&mut self) -> OdovifW<Ep1isrSpec> {
        OdovifW::new(self, 2)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn itrxif(&mut self) -> ItrxifW<Ep1isrSpec> {
        ItrxifW::new(self, 3)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    #[must_use]
    pub fn idtxif(&mut self) -> IdtxifW<Ep1isrSpec> {
        IdtxifW::new(self, 4)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    #[must_use]
    pub fn nakif(&mut self) -> NakifW<Ep1isrSpec> {
        NakifW::new(self, 5)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    #[must_use]
    pub fn stlif(&mut self) -> StlifW<Ep1isrSpec> {
        StlifW::new(self, 6)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    #[must_use]
    pub fn uerif(&mut self) -> UerifW<Ep1isrSpec> {
        UerifW::new(self, 7)
    }
}
#[doc = "EP1ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep1isrSpec;
impl crate::RegisterSpec for Ep1isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep1isr::R`](R) reader structure"]
impl crate::Readable for Ep1isrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep1isr::W`](W) writer structure"]
impl crate::Writable for Ep1isrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP1ISR to value 0"]
impl crate::Resettable for Ep1isrSpec {
    const RESET_VALUE: u32 = 0;
}
