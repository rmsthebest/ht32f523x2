#[doc = "Register `GPTM_CH0OCFR` reader"]
pub type R = crate::R<GptmCh0ocfrSpec>;
#[doc = "Register `GPTM_CH0OCFR` writer"]
pub type W = crate::W<GptmCh0ocfrSpec>;
#[doc = "Field `CH0OM` reader - CH0OM"]
pub type Ch0omR = crate::FieldReader;
#[doc = "Field `CH0OM` writer - CH0OM"]
pub type Ch0omW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH0PRE` reader - CH0PRE"]
pub type Ch0preR = crate::BitReader;
#[doc = "Field `CH0PRE` writer - CH0PRE"]
pub type Ch0preW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IMAE` reader - CH0IMAE"]
pub type Ch0imaeR = crate::BitReader;
#[doc = "Field `CH0IMAE` writer - CH0IMAE"]
pub type Ch0imaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OM3` reader - CH0OM3"]
pub type Ch0om3R = crate::BitReader;
#[doc = "Field `CH0OM3` writer - CH0OM3"]
pub type Ch0om3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    pub fn ch0om(&self) -> Ch0omR {
        Ch0omR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    pub fn ch0pre(&self) -> Ch0preR {
        Ch0preR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    pub fn ch0imae(&self) -> Ch0imaeR {
        Ch0imaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH0OM3"]
    #[inline(always)]
    pub fn ch0om3(&self) -> Ch0om3R {
        Ch0om3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch0om(&mut self) -> Ch0omW<GptmCh0ocfrSpec> {
        Ch0omW::new(self, 0)
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pre(&mut self) -> Ch0preW<GptmCh0ocfrSpec> {
        Ch0preW::new(self, 4)
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0imae(&mut self) -> Ch0imaeW<GptmCh0ocfrSpec> {
        Ch0imaeW::new(self, 5)
    }
    #[doc = "Bit 8 - CH0OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch0om3(&mut self) -> Ch0om3W<GptmCh0ocfrSpec> {
        Ch0om3W::new(self, 8)
    }
}
#[doc = "GPTM_CH0OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch0ocfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch0ocfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh0ocfrSpec;
impl crate::RegisterSpec for GptmCh0ocfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch0ocfr::R`](R) reader structure"]
impl crate::Readable for GptmCh0ocfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch0ocfr::W`](W) writer structure"]
impl crate::Writable for GptmCh0ocfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CH0OCFR to value 0"]
impl crate::Resettable for GptmCh0ocfrSpec {
    const RESET_VALUE: u32 = 0;
}
