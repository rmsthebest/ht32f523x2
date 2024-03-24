#[doc = "Register `APBCFGR` reader"]
pub type R = crate::R<ApbcfgrSpec>;
#[doc = "Register `APBCFGR` writer"]
pub type W = crate::W<ApbcfgrSpec>;
#[doc = "Field `ADCDIV` reader - ADCDIV"]
pub type AdcdivR = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - ADCDIV"]
pub type AdcdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    pub fn adcdiv(&self) -> AdcdivR {
        AdcdivR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv(&mut self) -> AdcdivW<ApbcfgrSpec> {
        AdcdivW::new(self, 16)
    }
}
#[doc = "APBCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcfgrSpec;
impl crate::RegisterSpec for ApbcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcfgr::R`](R) reader structure"]
impl crate::Readable for ApbcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcfgr::W`](W) writer structure"]
impl crate::Writable for ApbcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBCFGR to value 0"]
impl crate::Resettable for ApbcfgrSpec {
    const RESET_VALUE: u32 = 0;
}
