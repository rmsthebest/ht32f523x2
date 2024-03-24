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
    #[must_use]
    pub fn ahbpre(&mut self) -> AhbpreW<AhbcfgrSpec> {
        AhbpreW::new(self, 0)
    }
}
#[doc = "AHBCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbcfgrSpec;
impl crate::RegisterSpec for AhbcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbcfgr::R`](R) reader structure"]
impl crate::Readable for AhbcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbcfgr::W`](W) writer structure"]
impl crate::Writable for AhbcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBCFGR to value 0"]
impl crate::Resettable for AhbcfgrSpec {
    const RESET_VALUE: u32 = 0;
}
