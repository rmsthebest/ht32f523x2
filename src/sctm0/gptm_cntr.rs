#[doc = "Register `GPTM_CNTR` reader"]
pub type R = crate::R<GptmCntrSpec>;
#[doc = "Register `GPTM_CNTR` writer"]
pub type W = crate::W<GptmCntrSpec>;
#[doc = "Field `CNTV` reader - CNTV"]
pub type CntvR = crate::FieldReader<u16>;
#[doc = "Field `CNTV` writer - CNTV"]
pub type CntvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    pub fn cntv(&self) -> CntvR {
        CntvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    pub fn cntv(&mut self) -> CntvW<'_, GptmCntrSpec> {
        CntvW::new(self, 0)
    }
}
#[doc = "GPTM_CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCntrSpec;
impl crate::RegisterSpec for GptmCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_cntr::R`](R) reader structure"]
impl crate::Readable for GptmCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_cntr::W`](W) writer structure"]
impl crate::Writable for GptmCntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CNTR to value 0"]
impl crate::Resettable for GptmCntrSpec {}
