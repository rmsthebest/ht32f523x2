#[doc = "Register `GPTM_CH1ICFR` reader"]
pub type R = crate::R<GptmCh1icfrSpec>;
#[doc = "Register `GPTM_CH1ICFR` writer"]
pub type W = crate::W<GptmCh1icfrSpec>;
#[doc = "Field `TI1F` reader - TI1F"]
pub type Ti1fR = crate::FieldReader;
#[doc = "Field `TI1F` writer - TI1F"]
pub type Ti1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1CCS` reader - CH1CCS"]
pub type Ch1ccsR = crate::FieldReader;
#[doc = "Field `CH1CCS` writer - CH1CCS"]
pub type Ch1ccsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1PSC` reader - CH1PSC"]
pub type Ch1pscR = crate::FieldReader;
#[doc = "Field `CH1PSC` writer - CH1PSC"]
pub type Ch1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    pub fn ti1f(&self) -> Ti1fR {
        Ti1fR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    pub fn ch1ccs(&self) -> Ch1ccsR {
        Ch1ccsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    pub fn ch1psc(&self) -> Ch1pscR {
        Ch1pscR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    #[must_use]
    pub fn ti1f(&mut self) -> Ti1fW<GptmCh1icfrSpec> {
        Ti1fW::new(self, 0)
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccs(&mut self) -> Ch1ccsW<GptmCh1icfrSpec> {
        Ch1ccsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch1psc(&mut self) -> Ch1pscW<GptmCh1icfrSpec> {
        Ch1pscW::new(self, 18)
    }
}
#[doc = "GPTM_CH1ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch1icfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch1icfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh1icfrSpec;
impl crate::RegisterSpec for GptmCh1icfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch1icfr::R`](R) reader structure"]
impl crate::Readable for GptmCh1icfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch1icfr::W`](W) writer structure"]
impl crate::Writable for GptmCh1icfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH1ICFR to value 0"]
impl crate::Resettable for GptmCh1icfrSpec {
    const RESET_VALUE: u32 = 0;
}
