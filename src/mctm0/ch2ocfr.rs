#[doc = "Register `CH2OCFR` reader"]
pub type R = crate::R<Ch2ocfrSpec>;
#[doc = "Register `CH2OCFR` writer"]
pub type W = crate::W<Ch2ocfrSpec>;
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
    #[must_use]
    pub fn ch2om(&mut self) -> Ch2omW<Ch2ocfrSpec> {
        Ch2omW::new(self, 0)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pre(&mut self) -> Ch2preW<Ch2ocfrSpec> {
        Ch2preW::new(self, 4)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2imae(&mut self) -> Ch2imaeW<Ch2ocfrSpec> {
        Ch2imaeW::new(self, 5)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om3(&mut self) -> Ch2om3W<Ch2ocfrSpec> {
        Ch2om3W::new(self, 8)
    }
}
#[doc = "CH2OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ocfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ocfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ocfrSpec;
impl crate::RegisterSpec for Ch2ocfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2ocfr::R`](R) reader structure"]
impl crate::Readable for Ch2ocfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2ocfr::W`](W) writer structure"]
impl crate::Writable for Ch2ocfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2OCFR to value 0"]
impl crate::Resettable for Ch2ocfrSpec {
    const RESET_VALUE: u32 = 0;
}
