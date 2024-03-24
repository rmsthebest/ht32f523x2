#[doc = "Register `GPTM_CHCTR` reader"]
pub type R = crate::R<GptmChctrSpec>;
#[doc = "Register `GPTM_CHCTR` writer"]
pub type W = crate::W<GptmChctrSpec>;
#[doc = "Field `CH0E` reader - CH0E"]
pub type Ch0eR = crate::BitReader;
#[doc = "Field `CH0E` writer - CH0E"]
pub type Ch0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1E` reader - CH1E"]
pub type Ch1eR = crate::BitReader;
#[doc = "Field `CH1E` writer - CH1E"]
pub type Ch1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2E` reader - CH2E"]
pub type Ch2eR = crate::BitReader;
#[doc = "Field `CH2E` writer - CH2E"]
pub type Ch2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3E` reader - CH3E"]
pub type Ch3eR = crate::BitReader;
#[doc = "Field `CH3E` writer - CH3E"]
pub type Ch3eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&self) -> Ch0eR {
        Ch0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&self) -> Ch1eR {
        Ch1eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&self) -> Ch2eR {
        Ch2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&self) -> Ch3eR {
        Ch3eR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    #[must_use]
    pub fn ch0e(&mut self) -> Ch0eW<GptmChctrSpec> {
        Ch0eW::new(self, 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    #[must_use]
    pub fn ch1e(&mut self) -> Ch1eW<GptmChctrSpec> {
        Ch1eW::new(self, 2)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    #[must_use]
    pub fn ch2e(&mut self) -> Ch2eW<GptmChctrSpec> {
        Ch2eW::new(self, 4)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    #[must_use]
    pub fn ch3e(&mut self) -> Ch3eW<GptmChctrSpec> {
        Ch3eW::new(self, 6)
    }
}
#[doc = "GPTM_CHCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_chctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_chctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmChctrSpec;
impl crate::RegisterSpec for GptmChctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_chctr::R`](R) reader structure"]
impl crate::Readable for GptmChctrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_chctr::W`](W) writer structure"]
impl crate::Writable for GptmChctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CHCTR to value 0"]
impl crate::Resettable for GptmChctrSpec {
    const RESET_VALUE: u32 = 0;
}
