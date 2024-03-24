#[doc = "Register `GCSR` reader"]
pub type R = crate::R<GcsrSpec>;
#[doc = "Register `GCSR` writer"]
pub type W = crate::W<GcsrSpec>;
#[doc = "Field `PLLRDY` reader - PLLRDY"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - PLLRDY"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSERDY"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - HSERDY"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSIRDY"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - HSIRDY"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSERDY"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSERDY` writer - LSERDY"]
pub type LserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSIRDY"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSIRDY` writer - LSIRDY"]
pub type LsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdy(&mut self) -> PllrdyW<GcsrSpec> {
        PllrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HserdyW<GcsrSpec> {
        HserdyW::new(self, 2)
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HsirdyW<GcsrSpec> {
        HsirdyW::new(self, 3)
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LserdyW<GcsrSpec> {
        LserdyW::new(self, 4)
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LsirdyW<GcsrSpec> {
        LsirdyW::new(self, 5)
    }
}
#[doc = "GCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcsrSpec;
impl crate::RegisterSpec for GcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcsr::R`](R) reader structure"]
impl crate::Readable for GcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcsr::W`](W) writer structure"]
impl crate::Writable for GcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCSR to value 0"]
impl crate::Resettable for GcsrSpec {
    const RESET_VALUE: u32 = 0;
}
