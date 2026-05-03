#[doc = "Register `GPTM_CH2OCFR` reader"]
pub type R = crate::R<GptmCh2ocfrSpec>;
#[doc = "Register `GPTM_CH2OCFR` writer"]
pub type W = crate::W<GptmCh2ocfrSpec>;
#[doc = "Field `CH2OM` reader - CH2OM"]
pub type Ch2omR = crate::FieldReader;
#[doc = "Field `CH2OM` writer - CH2OM"]
pub type Ch2omW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2PRE` reader - CH2PRE"]
pub type Ch2preR = crate::BitReader;
#[doc = "Field `CH2PRE` writer - CH2PRE"]
pub type Ch2preW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2IMAE` reader - CH2IMAE"]
pub type Ch2imaeR = crate::BitReader;
#[doc = "Field `CH2IMAE` writer - CH2IMAE"]
pub type Ch2imaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OM3` reader - CH2OM3"]
pub type Ch2om3R = crate::BitReader;
#[doc = "Field `CH2OM3` writer - CH2OM3"]
pub type Ch2om3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> Ch2omR {
        Ch2omR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    pub fn ch2pre(&self) -> Ch2preR {
        Ch2preR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    pub fn ch2imae(&self) -> Ch2imaeR {
        Ch2imaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    pub fn ch2om3(&self) -> Ch2om3R {
        Ch2om3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> Ch2omW<'_, GptmCh2ocfrSpec> {
        Ch2omW::new(self, 0)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    pub fn ch2pre(&mut self) -> Ch2preW<'_, GptmCh2ocfrSpec> {
        Ch2preW::new(self, 4)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    pub fn ch2imae(&mut self) -> Ch2imaeW<'_, GptmCh2ocfrSpec> {
        Ch2imaeW::new(self, 5)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    pub fn ch2om3(&mut self) -> Ch2om3W<'_, GptmCh2ocfrSpec> {
        Ch2om3W::new(self, 8)
    }
}
#[doc = "GPTM_CH2OCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_ch2ocfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_ch2ocfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCh2ocfrSpec;
impl crate::RegisterSpec for GptmCh2ocfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ch2ocfr::R`](R) reader structure"]
impl crate::Readable for GptmCh2ocfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ch2ocfr::W`](W) writer structure"]
impl crate::Writable for GptmCh2ocfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CH2OCFR to value 0"]
impl crate::Resettable for GptmCh2ocfrSpec {}
