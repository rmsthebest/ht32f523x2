#[doc = "Register `CH0ICFR` reader"]
pub type R = crate::R<Ch0icfrSpec>;
#[doc = "Register `CH0ICFR` writer"]
pub type W = crate::W<Ch0icfrSpec>;
#[doc = "Field `TI0F` reader - TI0F"]
pub type Ti0fR = crate::FieldReader;
#[doc = "Field `TI0F` writer - TI0F"]
pub type Ti0fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH0CCS` reader - CH0CCS"]
pub type Ch0ccsR = crate::FieldReader;
#[doc = "Field `CH0CCS` writer - CH0CCS"]
pub type Ch0ccsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0PSC` reader - CH0PSC"]
pub type Ch0pscR = crate::FieldReader;
#[doc = "Field `CH0PSC` writer - CH0PSC"]
pub type Ch0pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TI0SRC` reader - TI0SRC"]
pub type Ti0srcR = crate::BitReader;
#[doc = "Field `TI0SRC` writer - TI0SRC"]
pub type Ti0srcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    pub fn ti0f(&self) -> Ti0fR {
        Ti0fR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    pub fn ch0ccs(&self) -> Ch0ccsR {
        Ch0ccsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    pub fn ch0psc(&self) -> Ch0pscR {
        Ch0pscR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    pub fn ti0src(&self) -> Ti0srcR {
        Ti0srcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    #[must_use]
    pub fn ti0f(&mut self) -> Ti0fW<Ch0icfrSpec> {
        Ti0fW::new(self, 0)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccs(&mut self) -> Ch0ccsW<Ch0icfrSpec> {
        Ch0ccsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch0psc(&mut self) -> Ch0pscW<Ch0icfrSpec> {
        Ch0pscW::new(self, 18)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ti0src(&mut self) -> Ti0srcW<Ch0icfrSpec> {
        Ti0srcW::new(self, 31)
    }
}
#[doc = "CH0ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0icfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0icfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0icfrSpec;
impl crate::RegisterSpec for Ch0icfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0icfr::R`](R) reader structure"]
impl crate::Readable for Ch0icfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0icfr::W`](W) writer structure"]
impl crate::Writable for Ch0icfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0ICFR to value 0"]
impl crate::Resettable for Ch0icfrSpec {
    const RESET_VALUE: u32 = 0;
}
