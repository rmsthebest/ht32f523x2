#[doc = "Register `GPTM_CH3ACR` reader"]
pub type R = crate::R<GptmCh3acrSpec>;
#[doc = "Register `GPTM_CH3ACR` writer"]
pub type W = crate::W<GptmCh3acrSpec>;
#[doc = "Field `CH3ACV` reader - CH3ACV"]
pub type Ch3acvR = crate::FieldReader<u16>;
#[doc = "Field `CH3ACV` writer - CH3ACV"]
pub type Ch3acvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    pub fn ch3acv(&self) -> Ch3acvR {
        Ch3acvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch3acv(&mut self) -> Ch3acvW<GptmCh3acrSpec> {
        Ch3acvW::new(self, 0)
    }
}
#[doc = "GPTM_CH3ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch3acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch3acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh3acrSpec;
impl crate::RegisterSpec for GptmCh3acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch3acr::R`](R) reader structure"]
impl crate::Readable for GptmCh3acrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch3acr::W`](W) writer structure"]
impl crate::Writable for GptmCh3acrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH3ACR to value 0"]
impl crate::Resettable for GptmCh3acrSpec {
    const RESET_VALUE: u32 = 0;
}
