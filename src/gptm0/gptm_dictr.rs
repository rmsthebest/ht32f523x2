#[doc = "Register `GPTM_DICTR` reader"]
pub type R = crate::R<GptmDictrSpec>;
#[doc = "Register `GPTM_DICTR` writer"]
pub type W = crate::W<GptmDictrSpec>;
#[doc = "Field `CH0CCIE` reader - CH0CCIE"]
pub type Ch0ccieR = crate::BitReader;
#[doc = "Field `CH0CCIE` writer - CH0CCIE"]
pub type Ch0ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCIE` reader - CH1CCIE"]
pub type Ch1ccieR = crate::BitReader;
#[doc = "Field `CH1CCIE` writer - CH1CCIE"]
pub type Ch1ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCIE` reader - CH2CCIE"]
pub type Ch2ccieR = crate::BitReader;
#[doc = "Field `CH2CCIE` writer - CH2CCIE"]
pub type Ch2ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCIE` reader - CH3CCIE"]
pub type Ch3ccieR = crate::BitReader;
#[doc = "Field `CH3CCIE` writer - CH3CCIE"]
pub type Ch3ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEVIE` reader - UEVIE"]
pub type UevieR = crate::BitReader;
#[doc = "Field `UEVIE` writer - UEVIE"]
pub type UevieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TevieR = crate::BitReader;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TevieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CCDE` reader - CH0CCDE"]
pub type Ch0ccdeR = crate::BitReader;
#[doc = "Field `CH0CCDE` writer - CH0CCDE"]
pub type Ch0ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCDE` reader - CH1CCDE"]
pub type Ch1ccdeR = crate::BitReader;
#[doc = "Field `CH1CCDE` writer - CH1CCDE"]
pub type Ch1ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCDE` reader - CH2CCDE"]
pub type Ch2ccdeR = crate::BitReader;
#[doc = "Field `CH2CCDE` writer - CH2CCDE"]
pub type Ch2ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCDE` reader - CH3CCDE"]
pub type Ch3ccdeR = crate::BitReader;
#[doc = "Field `CH3CCDE` writer - CH3CCDE"]
pub type Ch3ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEVDE` reader - UEVDE"]
pub type UevdeR = crate::BitReader;
#[doc = "Field `UEVDE` writer - UEVDE"]
pub type UevdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVDE` reader - TEVDE"]
pub type TevdeR = crate::BitReader;
#[doc = "Field `TEVDE` writer - TEVDE"]
pub type TevdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> Ch0ccieR {
        Ch0ccieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> Ch1ccieR {
        Ch1ccieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&self) -> Ch2ccieR {
        Ch2ccieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&self) -> Ch3ccieR {
        Ch3ccieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    pub fn uevie(&self) -> UevieR {
        UevieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TevieR {
        TevieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&self) -> Ch0ccdeR {
        Ch0ccdeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&self) -> Ch1ccdeR {
        Ch1ccdeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    pub fn ch2ccde(&self) -> Ch2ccdeR {
        Ch2ccdeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    pub fn ch3ccde(&self) -> Ch3ccdeR {
        Ch3ccdeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - UEVDE"]
    #[inline(always)]
    pub fn uevde(&self) -> UevdeR {
        UevdeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    pub fn tevde(&self) -> TevdeR {
        TevdeR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccie(&mut self) -> Ch0ccieW<GptmDictrSpec> {
        Ch0ccieW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccie(&mut self) -> Ch1ccieW<GptmDictrSpec> {
        Ch1ccieW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccie(&mut self) -> Ch2ccieW<GptmDictrSpec> {
        Ch2ccieW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccie(&mut self) -> Ch3ccieW<GptmDictrSpec> {
        Ch3ccieW::new(self, 3)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn uevie(&mut self) -> UevieW<GptmDictrSpec> {
        UevieW::new(self, 8)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn tevie(&mut self) -> TevieW<GptmDictrSpec> {
        TevieW::new(self, 10)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccde(&mut self) -> Ch0ccdeW<GptmDictrSpec> {
        Ch0ccdeW::new(self, 16)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccde(&mut self) -> Ch1ccdeW<GptmDictrSpec> {
        Ch1ccdeW::new(self, 17)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccde(&mut self) -> Ch2ccdeW<GptmDictrSpec> {
        Ch2ccdeW::new(self, 18)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccde(&mut self) -> Ch3ccdeW<GptmDictrSpec> {
        Ch3ccdeW::new(self, 19)
    }
    #[doc = "Bit 24 - UEVDE"]
    #[inline(always)]
    #[must_use]
    pub fn uevde(&mut self) -> UevdeW<GptmDictrSpec> {
        UevdeW::new(self, 24)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    #[must_use]
    pub fn tevde(&mut self) -> TevdeW<GptmDictrSpec> {
        TevdeW::new(self, 26)
    }
}
#[doc = "GPTM_DICTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_dictr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_dictr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmDictrSpec;
impl crate::RegisterSpec for GptmDictrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_dictr::R`](R) reader structure"]
impl crate::Readable for GptmDictrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_dictr::W`](W) writer structure"]
impl crate::Writable for GptmDictrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_DICTR to value 0"]
impl crate::Resettable for GptmDictrSpec {
    const RESET_VALUE: u32 = 0;
}
