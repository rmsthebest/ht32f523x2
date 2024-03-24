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
    #[must_use]
    pub fn ch2ccv(&mut self) -> Ch2ccvW<GptmCh2ccrSpec> {
        Ch2ccvW::new(self, 0)
    }
}
#[doc = "GPTM_CH2CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch2ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch2ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh2ccrSpec;
impl crate::RegisterSpec for GptmCh2ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch2ccr::R`](R) reader structure"]
impl crate::Readable for GptmCh2ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch2ccr::W`](W) writer structure"]
impl crate::Writable for GptmCh2ccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH2CCR to value 0"]
impl crate::Resettable for GptmCh2ccrSpec {
    const RESET_VALUE: u32 = 0;
}
