#[doc = "Register `HSICR` reader"]
pub type R = crate::R<HsicrSpec>;
#[doc = "Register `HSICR` writer"]
pub type W = crate::W<HsicrSpec>;
#[doc = "Field `TRIMEN` reader - TRIMEN"]
pub type TrimenR = crate::BitReader;
#[doc = "Field `TRIMEN` writer - TRIMEN"]
pub type TrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCEN` reader - ATCEN"]
pub type AtcenR = crate::BitReader;
#[doc = "Field `ATCEN` writer - ATCEN"]
pub type AtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSEL` reader - TMSEL"]
pub type TmselR = crate::BitReader;
#[doc = "Field `TMSEL` writer - TMSEL"]
pub type TmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCLKSEL` reader - REFCLKSEL"]
pub type RefclkselR = crate::BitReader;
#[doc = "Field `REFCLKSEL` writer - REFCLKSEL"]
pub type RefclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLOCK` reader - FLOCK"]
pub type FlockR = crate::BitReader;
#[doc = "Field `FLOCK` writer - FLOCK"]
pub type FlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIFINE` reader - HSIFINE"]
pub type HsifineR = crate::FieldReader;
#[doc = "Field `HSIFINE` writer - HSIFINE"]
pub type HsifineW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSICOARSE` reader - HSICOARSE"]
pub type HsicoarseR = crate::FieldReader;
#[doc = "Field `HSICOARSE` writer - HSICOARSE"]
pub type HsicoarseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    pub fn trimen(&self) -> TrimenR {
        TrimenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    pub fn atcen(&self) -> AtcenR {
        AtcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    pub fn tmsel(&self) -> TmselR {
        TmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    pub fn refclksel(&self) -> RefclkselR {
        RefclkselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    pub fn flock(&self) -> FlockR {
        FlockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    pub fn hsifine(&self) -> HsifineR {
        HsifineR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    pub fn hsicoarse(&self) -> HsicoarseR {
        HsicoarseR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    #[must_use]
    pub fn trimen(&mut self) -> TrimenW<HsicrSpec> {
        TrimenW::new(self, 0)
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    #[must_use]
    pub fn atcen(&mut self) -> AtcenW<HsicrSpec> {
        AtcenW::new(self, 1)
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn tmsel(&mut self) -> TmselW<HsicrSpec> {
        TmselW::new(self, 4)
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn refclksel(&mut self) -> RefclkselW<HsicrSpec> {
        RefclkselW::new(self, 5)
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn flock(&mut self) -> FlockW<HsicrSpec> {
        FlockW::new(self, 7)
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    #[must_use]
    pub fn hsifine(&mut self) -> HsifineW<HsicrSpec> {
        HsifineW::new(self, 16)
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    #[must_use]
    pub fn hsicoarse(&mut self) -> HsicoarseW<HsicrSpec> {
        HsicoarseW::new(self, 24)
    }
}
#[doc = "HSICR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsicrSpec;
impl crate::RegisterSpec for HsicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsicr::R`](R) reader structure"]
impl crate::Readable for HsicrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsicr::W`](W) writer structure"]
impl crate::Writable for HsicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSICR to value 0"]
impl crate::Resettable for HsicrSpec {
    const RESET_VALUE: u32 = 0;
}
