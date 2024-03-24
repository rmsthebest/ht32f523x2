#[doc = "Register `EP0ISR` reader"]
pub type R = crate::R<Ep0isrSpec>;
#[doc = "Register `EP0ISR` writer"]
pub type W = crate::W<Ep0isrSpec>;
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
#[doc = "Field `STRXIF` reader - STRXIF"]
pub type StrxifR = crate::BitReader;
#[doc = "Field `STRXIF` writer - STRXIF"]
pub type StrxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRXIF` reader - SDRXIF"]
pub type SdrxifR = crate::BitReader;
#[doc = "Field `SDRXIF` writer - SDRXIF"]
pub type SdrxifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDERIF` reader - SDERIF"]
pub type SderifR = crate::BitReader;
#[doc = "Field `SDERIF` writer - SDERIF"]
pub type SderifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZLRXIF` reader - ZLRXIF"]
pub type ZlrxifR = crate::BitReader;
#[doc = "Field `ZLRXIF` writer - ZLRXIF"]
pub type ZlrxifW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - STRXIF"]
    #[inline(always)]
    pub fn strxif(&self) -> StrxifR {
        StrxifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDRXIF"]
    #[inline(always)]
    pub fn sdrxif(&self) -> SdrxifR {
        SdrxifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDERIF"]
    #[inline(always)]
    pub fn sderif(&self) -> SderifR {
        SderifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ZLRXIF"]
    #[inline(always)]
    pub fn zlrxif(&self) -> ZlrxifR {
        ZlrxifR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn otrxif(&mut self) -> OtrxifW<Ep0isrSpec> {
        OtrxifW::new(self, 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn odrxif(&mut self) -> OdrxifW<Ep0isrSpec> {
        OdrxifW::new(self, 1)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    #[must_use]
    pub fn odovif(&mut self) -> OdovifW<Ep0isrSpec> {
        OdovifW::new(self, 2)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn itrxif(&mut self) -> ItrxifW<Ep0isrSpec> {
        ItrxifW::new(self, 3)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    #[must_use]
    pub fn idtxif(&mut self) -> IdtxifW<Ep0isrSpec> {
        IdtxifW::new(self, 4)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    #[must_use]
    pub fn nakif(&mut self) -> NakifW<Ep0isrSpec> {
        NakifW::new(self, 5)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    #[must_use]
    pub fn stlif(&mut self) -> StlifW<Ep0isrSpec> {
        StlifW::new(self, 6)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    #[must_use]
    pub fn uerif(&mut self) -> UerifW<Ep0isrSpec> {
        UerifW::new(self, 7)
    }
    #[doc = "Bit 8 - STRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn strxif(&mut self) -> StrxifW<Ep0isrSpec> {
        StrxifW::new(self, 8)
    }
    #[doc = "Bit 9 - SDRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn sdrxif(&mut self) -> SdrxifW<Ep0isrSpec> {
        SdrxifW::new(self, 9)
    }
    #[doc = "Bit 10 - SDERIF"]
    #[inline(always)]
    #[must_use]
    pub fn sderif(&mut self) -> SderifW<Ep0isrSpec> {
        SderifW::new(self, 10)
    }
    #[doc = "Bit 11 - ZLRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn zlrxif(&mut self) -> ZlrxifW<Ep0isrSpec> {
        ZlrxifW::new(self, 11)
    }
}
#[doc = "EP0ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0isrSpec;
impl crate::RegisterSpec for Ep0isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0isr::R`](R) reader structure"]
impl crate::Readable for Ep0isrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0isr::W`](W) writer structure"]
impl crate::Writable for Ep0isrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0ISR to value 0"]
impl crate::Resettable for Ep0isrSpec {
    const RESET_VALUE: u32 = 0;
}
