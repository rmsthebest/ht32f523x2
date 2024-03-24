#[doc = "Register `APBCCR1` reader"]
pub type R = crate::R<Apbccr1Spec>;
#[doc = "Register `APBCCR1` writer"]
pub type W = crate::W<Apbccr1Spec>;
#[doc = "Field `MCTM0EN` reader - MCTM0EN"]
pub type Mctm0enR = crate::BitReader;
#[doc = "Field `MCTM0EN` writer - MCTM0EN"]
pub type Mctm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTEN` reader - WDTEN"]
pub type WdtenR = crate::BitReader;
#[doc = "Field `WDTEN` writer - WDTEN"]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPREN` reader - BKPREN"]
pub type BkprenR = crate::BitReader;
#[doc = "Field `BKPREN` writer - BKPREN"]
pub type BkprenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTM0EN` reader - GPTM0EN"]
pub type Gptm0enR = crate::BitReader;
#[doc = "Field `GPTM0EN` writer - GPTM0EN"]
pub type Gptm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTM1EN` reader - GPTM1EN"]
pub type Gptm1enR = crate::BitReader;
#[doc = "Field `GPTM1EN` writer - GPTM1EN"]
pub type Gptm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFTM0EN` reader - BFTM0EN"]
pub type Bftm0enR = crate::BitReader;
#[doc = "Field `BFTM0EN` writer - BFTM0EN"]
pub type Bftm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFTM1EN` reader - BFTM1EN"]
pub type Bftm1enR = crate::BitReader;
#[doc = "Field `BFTM1EN` writer - BFTM1EN"]
pub type Bftm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - CMPEN"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - CMPEN"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADCEN"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADCEN"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTM0EN` reader - SCTM0EN"]
pub type Sctm0enR = crate::BitReader;
#[doc = "Field `SCTM0EN` writer - SCTM0EN"]
pub type Sctm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTM1EN` reader - SCTM1EN"]
pub type Sctm1enR = crate::BitReader;
#[doc = "Field `SCTM1EN` writer - SCTM1EN"]
pub type Sctm1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCTM0EN"]
    #[inline(always)]
    pub fn mctm0en(&self) -> Mctm0enR {
        Mctm0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - BKPREN"]
    #[inline(always)]
    pub fn bkpren(&self) -> BkprenR {
        BkprenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&self) -> Gptm0enR {
        Gptm0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&self) -> Gptm1enR {
        Gptm1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    pub fn bftm0en(&self) -> Bftm0enR {
        Bftm0enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    pub fn bftm1en(&self) -> Bftm1enR {
        Bftm1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    pub fn sctm0en(&self) -> Sctm0enR {
        Sctm0enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    pub fn sctm1en(&self) -> Sctm1enR {
        Sctm1enR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn mctm0en(&mut self) -> Mctm0enW<Apbccr1Spec> {
        Mctm0enW::new(self, 0)
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WdtenW<Apbccr1Spec> {
        WdtenW::new(self, 4)
    }
    #[doc = "Bit 6 - BKPREN"]
    #[inline(always)]
    #[must_use]
    pub fn bkpren(&mut self) -> BkprenW<Apbccr1Spec> {
        BkprenW::new(self, 6)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0en(&mut self) -> Gptm0enW<Apbccr1Spec> {
        Gptm0enW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1en(&mut self) -> Gptm1enW<Apbccr1Spec> {
        Gptm1enW::new(self, 9)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0en(&mut self) -> Bftm0enW<Apbccr1Spec> {
        Bftm0enW::new(self, 16)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1en(&mut self) -> Bftm1enW<Apbccr1Spec> {
        Bftm1enW::new(self, 17)
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<Apbccr1Spec> {
        CmpenW::new(self, 22)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<Apbccr1Spec> {
        AdcenW::new(self, 24)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0en(&mut self) -> Sctm0enW<Apbccr1Spec> {
        Sctm0enW::new(self, 28)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1en(&mut self) -> Sctm1enW<Apbccr1Spec> {
        Sctm1enW::new(self, 29)
    }
}
#[doc = "APBCCR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbccr1Spec;
impl crate::RegisterSpec for Apbccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbccr1::R`](R) reader structure"]
impl crate::Readable for Apbccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbccr1::W`](W) writer structure"]
impl crate::Writable for Apbccr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBCCR1 to value 0"]
impl crate::Resettable for Apbccr1Spec {
    const RESET_VALUE: u32 = 0;
}
