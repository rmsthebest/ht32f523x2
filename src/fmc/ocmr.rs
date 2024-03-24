#[doc = "Register `OCMR` reader"]
pub type R = crate::R<OcmrSpec>;
#[doc = "Register `OCMR` writer"]
pub type W = crate::W<OcmrSpec>;
#[doc = "Field `CMD` reader - CMD"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - CMD"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<OcmrSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "OCMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmrSpec;
impl crate::RegisterSpec for OcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocmr::R`](R) reader structure"]
impl crate::Readable for OcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`ocmr::W`](W) writer structure"]
impl crate::Writable for OcmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCMR to value 0"]
impl crate::Resettable for OcmrSpec {
    const RESET_VALUE: u32 = 0;
}
