#[doc = "Register `AHBCFGR` reader"]
pub type R = crate::R<AhbcfgrSpec>;
#[doc = "Register `AHBCFGR` writer"]
pub type W = crate::W<AhbcfgrSpec>;
#[doc = "Field `AHBPRE` reader - AHBPRE"]
pub type AhbpreR = crate::FieldReader;
#[doc = "Field `AHBPRE` writer - AHBPRE"]
pub type AhbpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&self) -> AhbpreR {
        AhbpreR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHBPRE"]
    #[inline(always)]
    pub fn ahbpre(&mut self) -> AhbpreW<'_, AhbcfgrSpec> {
        AhbpreW::new(self, 0)
    }
}
#[doc = "AHBCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbcfgrSpec;
impl crate::RegisterSpec for AhbcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbcfgr::R`](R) reader structure"]
impl crate::Readable for AhbcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbcfgr::W`](W) writer structure"]
impl crate::Writable for AhbcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCFGR to value 0"]
impl crate::Resettable for AhbcfgrSpec {}
