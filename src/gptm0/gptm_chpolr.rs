#[doc = "Register `GPTM_CHPOLR` reader"]
pub type R = crate::R<GptmChpolrSpec>;
#[doc = "Register `GPTM_CHPOLR` writer"]
pub type W = crate::W<GptmChpolrSpec>;
#[doc = "Field `CH0P` reader - CH0P"]
pub type Ch0pR = crate::BitReader;
#[doc = "Field `CH0P` writer - CH0P"]
pub type Ch0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1P` reader - CH1P"]
pub type Ch1pR = crate::BitReader;
#[doc = "Field `CH1P` writer - CH1P"]
pub type Ch1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2P` reader - CH2P"]
pub type Ch2pR = crate::BitReader;
#[doc = "Field `CH2P` writer - CH2P"]
pub type Ch2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3P` reader - CH3P"]
pub type Ch3pR = crate::BitReader;
#[doc = "Field `CH3P` writer - CH3P"]
pub type Ch3pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&self) -> Ch2pR {
        Ch2pR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&self) -> Ch3pR {
        Ch3pR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> Ch0pW<GptmChpolrSpec> {
        Ch0pW::new(self, 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> Ch1pW<GptmChpolrSpec> {
        Ch1pW::new(self, 2)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> Ch2pW<GptmChpolrSpec> {
        Ch2pW::new(self, 4)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> Ch3pW<GptmChpolrSpec> {
        Ch3pW::new(self, 6)
    }
}
#[doc = "GPTM_CHPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_chpolr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_chpolr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmChpolrSpec;
impl crate::RegisterSpec for GptmChpolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_chpolr::R`](R) reader structure"]
impl crate::Readable for GptmChpolrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_chpolr::W`](W) writer structure"]
impl crate::Writable for GptmChpolrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CHPOLR to value 0"]
impl crate::Resettable for GptmChpolrSpec {
    const RESET_VALUE: u32 = 0;
}
