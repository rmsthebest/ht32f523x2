#[doc = "Register `GPTM_CH1OCFR` reader"]
pub type R = crate::R<GptmCh1ocfrSpec>;
#[doc = "Register `GPTM_CH1OCFR` writer"]
pub type W = crate::W<GptmCh1ocfrSpec>;
#[doc = "Field `CH1OM` reader - CH1OM"]
pub type Ch1omR = crate::FieldReader;
#[doc = "Field `CH1OM` writer - CH1OM"]
pub type Ch1omW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1PRE` reader - CH1PRE"]
pub type Ch1preR = crate::BitReader;
#[doc = "Field `CH1PRE` writer - CH1PRE"]
pub type Ch1preW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1IMAE` reader - CH1IMAE"]
pub type Ch1imaeR = crate::BitReader;
#[doc = "Field `CH1IMAE` writer - CH1IMAE"]
pub type Ch1imaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OM3` reader - CH1OM3"]
pub type Ch1om3R = crate::BitReader;
#[doc = "Field `CH1OM3` writer - CH1OM3"]
pub type Ch1om3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&self) -> Ch1omR {
        Ch1omR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    pub fn ch1pre(&self) -> Ch1preR {
        Ch1preR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    pub fn ch1imae(&self) -> Ch1imaeR {
        Ch1imaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH1OM3"]
    #[inline(always)]
    pub fn ch1om3(&self) -> Ch1om3R {
        Ch1om3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH1OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> Ch1omW<GptmCh1ocfrSpec> {
        Ch1omW::new(self, 0)
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pre(&mut self) -> Ch1preW<GptmCh1ocfrSpec> {
        Ch1preW::new(self, 4)
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1imae(&mut self) -> Ch1imaeW<GptmCh1ocfrSpec> {
        Ch1imaeW::new(self, 5)
    }
    #[doc = "Bit 8 - CH1OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om3(&mut self) -> Ch1om3W<GptmCh1ocfrSpec> {
        Ch1om3W::new(self, 8)
    }
}
#[doc = "GPTM_CH1OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch1ocfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch1ocfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh1ocfrSpec;
impl crate::RegisterSpec for GptmCh1ocfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch1ocfr::R`](R) reader structure"]
impl crate::Readable for GptmCh1ocfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch1ocfr::W`](W) writer structure"]
impl crate::Writable for GptmCh1ocfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH1OCFR to value 0"]
impl crate::Resettable for GptmCh1ocfrSpec {
    const RESET_VALUE: u32 = 0;
}
