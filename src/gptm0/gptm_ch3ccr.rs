#[doc = "Register `GPTM_CH3CCR` reader"]
pub type R = crate::R<GptmCh3ccrSpec>;
#[doc = "Register `GPTM_CH3CCR` writer"]
pub type W = crate::W<GptmCh3ccrSpec>;
#[doc = "Field `CH3CCV` reader - CH3CCV"]
pub type Ch3ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH3CCV` writer - CH3CCV"]
pub type Ch3ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&self) -> Ch3ccvR {
        Ch3ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&mut self) -> Ch3ccvW<'_, GptmCh3ccrSpec> {
        Ch3ccvW::new(self, 0)
    }
}
#[doc = "GPTM_CH3CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_ch3ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_ch3ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh3ccrSpec;
impl crate::RegisterSpec for GptmCh3ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch3ccr::R`](R) reader structure"]
impl crate::Readable for GptmCh3ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch3ccr::W`](W) writer structure"]
impl crate::Writable for GptmCh3ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CH3CCR to value 0"]
impl crate::Resettable for GptmCh3ccrSpec {}
