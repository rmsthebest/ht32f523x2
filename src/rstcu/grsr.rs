#[doc = "Register `GRSR` reader"]
pub type R = crate::R<GrsrSpec>;
#[doc = "Register `GRSR` writer"]
pub type W = crate::W<GrsrSpec>;
#[doc = "Field `NVICRSTF` reader - NVICRSTF"]
pub type NvicrstfR = crate::BitReader;
#[doc = "Field `NVICRSTF` writer - NVICRSTF"]
pub type NvicrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSTF` reader - EXTRSTF"]
pub type ExtrstfR = crate::BitReader;
#[doc = "Field `EXTRSTF` writer - EXTRSTF"]
pub type ExtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTRSTF` reader - WDTRSTF"]
pub type WdtrstfR = crate::BitReader;
#[doc = "Field `WDTRSTF` writer - WDTRSTF"]
pub type WdtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORSTF` reader - PORSTF"]
pub type PorstfR = crate::BitReader;
#[doc = "Field `PORSTF` writer - PORSTF"]
pub type PorstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    pub fn nvicrstf(&self) -> NvicrstfR {
        NvicrstfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    pub fn extrstf(&self) -> ExtrstfR {
        ExtrstfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WdtrstfR {
        WdtrstfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    pub fn porstf(&self) -> PorstfR {
        PorstfR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn nvicrstf(&mut self) -> NvicrstfW<GrsrSpec> {
        NvicrstfW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn extrstf(&mut self) -> ExtrstfW<GrsrSpec> {
        ExtrstfW::new(self, 1)
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrstf(&mut self) -> WdtrstfW<GrsrSpec> {
        WdtrstfW::new(self, 2)
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    #[must_use]
    pub fn porstf(&mut self) -> PorstfW<GrsrSpec> {
        PorstfW::new(self, 3)
    }
}
#[doc = "GRSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrsrSpec;
impl crate::RegisterSpec for GrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grsr::R`](R) reader structure"]
impl crate::Readable for GrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`grsr::W`](W) writer structure"]
impl crate::Writable for GrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSR to value 0"]
impl crate::Resettable for GrsrSpec {
    const RESET_VALUE: u32 = 0;
}
