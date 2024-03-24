#[doc = "Register `GPTM_CH3ICFR` reader"]
pub type R = crate::R<GptmCh3icfrSpec>;
#[doc = "Register `GPTM_CH3ICFR` writer"]
pub type W = crate::W<GptmCh3icfrSpec>;
#[doc = "Field `TI3F` reader - TI3F"]
pub type Ti3fR = crate::FieldReader;
#[doc = "Field `TI3F` writer - TI3F"]
pub type Ti3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH3CCS` reader - CH3CCS"]
pub type Ch3ccsR = crate::FieldReader;
#[doc = "Field `CH3CCS` writer - CH3CCS"]
pub type Ch3ccsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3PSC` reader - CH3PSC"]
pub type Ch3pscR = crate::FieldReader;
#[doc = "Field `CH3PSC` writer - CH3PSC"]
pub type Ch3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    pub fn ti3f(&self) -> Ti3fR {
        Ti3fR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    pub fn ch3ccs(&self) -> Ch3ccsR {
        Ch3ccsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    pub fn ch3psc(&self) -> Ch3pscR {
        Ch3pscR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    #[must_use]
    pub fn ti3f(&mut self) -> Ti3fW<GptmCh3icfrSpec> {
        Ti3fW::new(self, 0)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccs(&mut self) -> Ch3ccsW<GptmCh3icfrSpec> {
        Ch3ccsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch3psc(&mut self) -> Ch3pscW<GptmCh3icfrSpec> {
        Ch3pscW::new(self, 18)
    }
}
#[doc = "GPTM_CH3ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch3icfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch3icfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh3icfrSpec;
impl crate::RegisterSpec for GptmCh3icfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch3icfr::R`](R) reader structure"]
impl crate::Readable for GptmCh3icfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch3icfr::W`](W) writer structure"]
impl crate::Writable for GptmCh3icfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH3ICFR to value 0"]
impl crate::Resettable for GptmCh3icfrSpec {
    const RESET_VALUE: u32 = 0;
}
