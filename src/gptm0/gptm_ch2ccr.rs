#[doc = "Register `GPTM_CH2CCR` reader"]
pub type R = crate::R<GptmCh2ccrSpec>;
#[doc = "Register `GPTM_CH2CCR` writer"]
pub type W = crate::W<GptmCh2ccrSpec>;
#[doc = "Field `CH2CCV` reader - CH2CCV"]
pub type Ch2ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH2CCV` writer - CH2CCV"]
pub type Ch2ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&self) -> Ch2ccvR {
        Ch2ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&mut self) -> Ch2ccvW<'_, GptmCh2ccrSpec> {
        Ch2ccvW::new(self, 0)
    }
}
#[doc = "GPTM_CH2CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_ch2ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_ch2ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh2ccrSpec;
impl crate::RegisterSpec for GptmCh2ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch2ccr::R`](R) reader structure"]
impl crate::Readable for GptmCh2ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch2ccr::W`](W) writer structure"]
impl crate::Writable for GptmCh2ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CH2CCR to value 0"]
impl crate::Resettable for GptmCh2ccrSpec {}
