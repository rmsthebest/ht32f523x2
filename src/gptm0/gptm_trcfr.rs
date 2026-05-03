#[doc = "Register `GPTM_TRCFR` reader"]
pub type R = crate::R<GptmTrcfrSpec>;
#[doc = "Register `GPTM_TRCFR` writer"]
pub type W = crate::W<GptmTrcfrSpec>;
#[doc = "Field `TRSEL` reader - TRSEL"]
pub type TrselR = crate::FieldReader;
#[doc = "Field `TRSEL` writer - TRSEL"]
pub type TrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TrselR {
        TrselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&mut self) -> TrselW<'_, GptmTrcfrSpec> {
        TrselW::new(self, 0)
    }
}
#[doc = "GPTM_TRCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_trcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_trcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmTrcfrSpec;
impl crate::RegisterSpec for GptmTrcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_trcfr::R`](R) reader structure"]
impl crate::Readable for GptmTrcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_trcfr::W`](W) writer structure"]
impl crate::Writable for GptmTrcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_TRCFR to value 0"]
impl crate::Resettable for GptmTrcfrSpec {}
