#[doc = "Register `GPTM_CH0ACR` reader"]
pub type R = crate::R<GptmCh0acrSpec>;
#[doc = "Register `GPTM_CH0ACR` writer"]
pub type W = crate::W<GptmCh0acrSpec>;
#[doc = "Field `CH0ACV` reader - CH0ACV"]
pub type Ch0acvR = crate::FieldReader<u16>;
#[doc = "Field `CH0ACV` writer - CH0ACV"]
pub type Ch0acvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    pub fn ch0acv(&self) -> Ch0acvR {
        Ch0acvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch0acv(&mut self) -> Ch0acvW<GptmCh0acrSpec> {
        Ch0acvW::new(self, 0)
    }
}
#[doc = "GPTM_CH0ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch0acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch0acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh0acrSpec;
impl crate::RegisterSpec for GptmCh0acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch0acr::R`](R) reader structure"]
impl crate::Readable for GptmCh0acrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch0acr::W`](W) writer structure"]
impl crate::Writable for GptmCh0acrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH0ACR to value 0"]
impl crate::Resettable for GptmCh0acrSpec {
    const RESET_VALUE: u32 = 0;
}
