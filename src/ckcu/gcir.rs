#[doc = "Register `GCIR` reader"]
pub type R = crate::R<GcirSpec>;
#[doc = "Register `GCIR` writer"]
pub type W = crate::W<GcirSpec>;
#[doc = "Field `CKSF` reader - CKSF"]
pub type CksfR = crate::BitReader;
#[doc = "Field `CKSF` writer - CKSF"]
pub type CksfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYF` reader - PLLRDYF"]
pub type PllrdyfR = crate::BitReader;
#[doc = "Field `PLLRDYF` writer - PLLRDYF"]
pub type PllrdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSERDYF"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSERDYF"]
pub type HserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSIRDYF"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSIRDYF"]
pub type HsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYF` reader - LSERDYF"]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` writer - LSERDYF"]
pub type LserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYF` reader - LSIRDYF"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSIRDYF"]
pub type LsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSIE` reader - CKSIE"]
pub type CksieR = crate::BitReader;
#[doc = "Field `CKSIE` writer - CKSIE"]
pub type CksieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - PLLRDYIE"]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - PLLRDYIE"]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSERDYIE"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSERDYIE"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSIRDYIE"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSIRDYIE"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSERDYIE"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSERDYIE"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYIE` reader - LSIRDYIE"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSIRDYIE"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    pub fn cksf(&self) -> CksfR {
        CksfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    pub fn cksie(&self) -> CksieR {
        CksieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    #[must_use]
    pub fn cksf(&mut self) -> CksfW<GcirSpec> {
        CksfW::new(self, 0)
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyf(&mut self) -> PllrdyfW<GcirSpec> {
        PllrdyfW::new(self, 2)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HserdyfW<GcirSpec> {
        HserdyfW::new(self, 3)
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HsirdyfW<GcirSpec> {
        HsirdyfW::new(self, 4)
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LserdyfW<GcirSpec> {
        LserdyfW::new(self, 5)
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LsirdyfW<GcirSpec> {
        LsirdyfW::new(self, 6)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    #[must_use]
    pub fn cksie(&mut self) -> CksieW<GcirSpec> {
        CksieW::new(self, 16)
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PllrdyieW<GcirSpec> {
        PllrdyieW::new(self, 18)
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HserdyieW<GcirSpec> {
        HserdyieW::new(self, 19)
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HsirdyieW<GcirSpec> {
        HsirdyieW::new(self, 20)
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LserdyieW<GcirSpec> {
        LserdyieW::new(self, 21)
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LsirdyieW<GcirSpec> {
        LsirdyieW::new(self, 22)
    }
}
#[doc = "GCIR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcirSpec;
impl crate::RegisterSpec for GcirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcir::R`](R) reader structure"]
impl crate::Readable for GcirSpec {}
#[doc = "`write(|w| ..)` method takes [`gcir::W`](W) writer structure"]
impl crate::Writable for GcirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCIR to value 0"]
impl crate::Resettable for GcirSpec {
    const RESET_VALUE: u32 = 0;
}
