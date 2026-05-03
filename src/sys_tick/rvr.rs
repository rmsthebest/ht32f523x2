#[doc = "Register `RVR` reader"]
pub type R = crate::R<RvrSpec>;
#[doc = "Register `RVR` writer"]
pub type W = crate::W<RvrSpec>;
#[doc = "Field `RELOAD` reader - RELOAD"]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - RELOAD"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RELOAD"]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<'_, RvrSpec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "RVR\n\nYou can [`read`](crate::Reg::read) this register and get [`rvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RvrSpec;
impl crate::RegisterSpec for RvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rvr::R`](R) reader structure"]
impl crate::Readable for RvrSpec {}
#[doc = "`write(|w| ..)` method takes [`rvr::W`](W) writer structure"]
impl crate::Writable for RvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RVR to value 0"]
impl crate::Resettable for RvrSpec {}
