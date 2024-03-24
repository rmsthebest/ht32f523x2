#[doc = "Register `CH3OCFR` reader"]
pub type R = crate::R<Ch3ocfrSpec>;
#[doc = "Register `CH3OCFR` writer"]
pub type W = crate::W<Ch3ocfrSpec>;
#[doc = "Field `CH3OM` reader - CH3OM"]
pub type Ch3omR = crate::FieldReader;
#[doc = "Field `CH3OM` writer - CH3OM"]
pub type Ch3omW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH3PRE` reader - CH3PRE"]
pub type Ch3preR = crate::BitReader;
#[doc = "Field `CH3PRE` writer - CH3PRE"]
pub type Ch3preW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3IMAE` reader - CH3IMAE"]
pub type Ch3imaeR = crate::BitReader;
#[doc = "Field `CH3IMAE` writer - CH3IMAE"]
pub type Ch3imaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OM3` reader - CH3OM3"]
pub type Ch3om3R = crate::BitReader;
#[doc = "Field `CH3OM3` writer - CH3OM3"]
pub type Ch3om3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> Ch3omR {
        Ch3omR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    pub fn ch3pre(&self) -> Ch3preR {
        Ch3preR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    pub fn ch3imae(&self) -> Ch3imaeR {
        Ch3imaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    pub fn ch3om3(&self) -> Ch3om3R {
        Ch3om3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> Ch3omW<Ch3ocfrSpec> {
        Ch3omW::new(self, 0)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pre(&mut self) -> Ch3preW<Ch3ocfrSpec> {
        Ch3preW::new(self, 4)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3imae(&mut self) -> Ch3imaeW<Ch3ocfrSpec> {
        Ch3imaeW::new(self, 5)
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om3(&mut self) -> Ch3om3W<Ch3ocfrSpec> {
        Ch3om3W::new(self, 8)
    }
}
#[doc = "CH3OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ocfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ocfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ocfrSpec;
impl crate::RegisterSpec for Ch3ocfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ocfr::R`](R) reader structure"]
impl crate::Readable for Ch3ocfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ocfr::W`](W) writer structure"]
impl crate::Writable for Ch3ocfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3OCFR to value 0"]
impl crate::Resettable for Ch3ocfrSpec {
    const RESET_VALUE: u32 = 0;
}
