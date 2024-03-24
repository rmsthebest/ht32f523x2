#[doc = "Register `GPTM_CH0CCR` reader"]
pub type R = crate::R<GptmCh0ccrSpec>;
#[doc = "Register `GPTM_CH0CCR` writer"]
pub type W = crate::W<GptmCh0ccrSpec>;
#[doc = "Field `CH0CCV` reader - CH0CCV"]
pub type Ch0ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH0CCV` writer - CH0CCV"]
pub type Ch0ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH0CCV"]
    #[inline(always)]
    pub fn ch0ccv(&self) -> Ch0ccvR {
        Ch0ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0CCV"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccv(&mut self) -> Ch0ccvW<GptmCh0ccrSpec> {
        Ch0ccvW::new(self, 0)
    }
}
#[doc = "GPTM_CH0CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch0ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch0ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh0ccrSpec;
impl crate::RegisterSpec for GptmCh0ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch0ccr::R`](R) reader structure"]
impl crate::Readable for GptmCh0ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch0ccr::W`](W) writer structure"]
impl crate::Writable for GptmCh0ccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH0CCR to value 0"]
impl crate::Resettable for GptmCh0ccrSpec {
    const RESET_VALUE: u32 = 0;
}
