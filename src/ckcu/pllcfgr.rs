#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PllcfgrSpec>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PllcfgrSpec>;
#[doc = "Field `POTD` reader - POTD"]
pub type PotdR = crate::FieldReader;
#[doc = "Field `POTD` writer - POTD"]
pub type PotdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PFBD` reader - PFBD"]
pub type PfbdR = crate::FieldReader;
#[doc = "Field `PFBD` writer - PFBD"]
pub type PfbdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    pub fn potd(&self) -> PotdR {
        PotdR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:26 - PFBD"]
    #[inline(always)]
    pub fn pfbd(&self) -> PfbdR {
        PfbdR::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    #[must_use]
    pub fn potd(&mut self) -> PotdW<PllcfgrSpec> {
        PotdW::new(self, 21)
    }
    #[doc = "Bits 23:26 - PFBD"]
    #[inline(always)]
    #[must_use]
    pub fn pfbd(&mut self) -> PfbdW<PllcfgrSpec> {
        PfbdW::new(self, 23)
    }
}
#[doc = "PLLCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgrSpec;
impl crate::RegisterSpec for PllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PllcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0"]
impl crate::Resettable for PllcfgrSpec {
    const RESET_VALUE: u32 = 0;
}
