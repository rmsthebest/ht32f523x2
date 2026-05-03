#[doc = "Register `EP4ISR` reader"]
pub type R = crate::R<Ep4isrSpec>;
#[doc = "Register `EP4ISR` writer"]
pub type W = crate::W<Ep4isrSpec>;
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
    pub fn otrxif(&mut self) -> OtrxifW<'_, Ep4isrSpec> {
        OtrxifW::new(self, 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    pub fn odrxif(&mut self) -> OdrxifW<'_, Ep4isrSpec> {
        OdrxifW::new(self, 1)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    pub fn odovif(&mut self) -> OdovifW<'_, Ep4isrSpec> {
        OdovifW::new(self, 2)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    pub fn itrxif(&mut self) -> ItrxifW<'_, Ep4isrSpec> {
        ItrxifW::new(self, 3)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    pub fn idtxif(&mut self) -> IdtxifW<'_, Ep4isrSpec> {
        IdtxifW::new(self, 4)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    pub fn nakif(&mut self) -> NakifW<'_, Ep4isrSpec> {
        NakifW::new(self, 5)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    pub fn stlif(&mut self) -> StlifW<'_, Ep4isrSpec> {
        StlifW::new(self, 6)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    pub fn uerif(&mut self) -> UerifW<'_, Ep4isrSpec> {
        UerifW::new(self, 7)
    }
}
#[doc = "EP4ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4isrSpec;
impl crate::RegisterSpec for Ep4isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep4isr::R`](R) reader structure"]
impl crate::Readable for Ep4isrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep4isr::W`](W) writer structure"]
impl crate::Writable for Ep4isrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP4ISR to value 0"]
impl crate::Resettable for Ep4isrSpec {}
