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
    pub fn cmd(&mut self) -> CmdW<'_, OcmrSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "OCMR\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmrSpec;
impl crate::RegisterSpec for OcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocmr::R`](R) reader structure"]
impl crate::Readable for OcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`ocmr::W`](W) writer structure"]
impl crate::Writable for OcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCMR to value 0"]
impl crate::Resettable for OcmrSpec {}
