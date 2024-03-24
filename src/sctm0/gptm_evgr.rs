#[doc = "Register `GPTM_EVGR` reader"]
pub type R = crate::R<GptmEvgrSpec>;
#[doc = "Register `GPTM_EVGR` writer"]
pub type W = crate::W<GptmEvgrSpec>;
#[doc = "Field `CH0CCG` reader - CH0CCG"]
pub type Ch0ccgR = crate::BitReader;
#[doc = "Field `CH0CCG` writer - CH0CCG"]
pub type Ch0ccgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCG` reader - CH1CCG"]
pub type Ch1ccgR = crate::BitReader;
#[doc = "Field `CH1CCG` writer - CH1CCG"]
pub type Ch1ccgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCG` reader - CH2CCG"]
pub type Ch2ccgR = crate::BitReader;
#[doc = "Field `CH2CCG` writer - CH2CCG"]
pub type Ch2ccgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCG` reader - CH3CCG"]
pub type Ch3ccgR = crate::BitReader;
#[doc = "Field `CH3CCG` writer - CH3CCG"]
pub type Ch3ccgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEVG` reader - UEVG"]
pub type UevgR = crate::BitReader;
#[doc = "Field `UEVG` writer - UEVG"]
pub type UevgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVG` reader - TEVG"]
pub type TevgR = crate::BitReader;
#[doc = "Field `TEVG` writer - TEVG"]
pub type TevgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    pub fn ch0ccg(&self) -> Ch0ccgR {
        Ch0ccgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    pub fn ch1ccg(&self) -> Ch1ccgR {
        Ch1ccgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    pub fn ch2ccg(&self) -> Ch2ccgR {
        Ch2ccgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    pub fn ch3ccg(&self) -> Ch3ccgR {
        Ch3ccgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&self) -> UevgR {
        UevgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&self) -> TevgR {
        TevgR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccg(&mut self) -> Ch0ccgW<GptmEvgrSpec> {
        Ch0ccgW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccg(&mut self) -> Ch1ccgW<GptmEvgrSpec> {
        Ch1ccgW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccg(&mut self) -> Ch2ccgW<GptmEvgrSpec> {
        Ch2ccgW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccg(&mut self) -> Ch3ccgW<GptmEvgrSpec> {
        Ch3ccgW::new(self, 3)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    #[must_use]
    pub fn uevg(&mut self) -> UevgW<GptmEvgrSpec> {
        UevgW::new(self, 8)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    #[must_use]
    pub fn tevg(&mut self) -> TevgW<GptmEvgrSpec> {
        TevgW::new(self, 10)
    }
}
#[doc = "GPTM_EVGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_evgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_evgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmEvgrSpec;
impl crate::RegisterSpec for GptmEvgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_evgr::R`](R) reader structure"]
impl crate::Readable for GptmEvgrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_evgr::W`](W) writer structure"]
impl crate::Writable for GptmEvgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_EVGR to value 0"]
impl crate::Resettable for GptmEvgrSpec {
    const RESET_VALUE: u32 = 0;
}
