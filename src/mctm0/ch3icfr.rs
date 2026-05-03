#[doc = "Register `CH3ICFR` reader"]
pub type R = crate::R<Ch3icfrSpec>;
#[doc = "Register `CH3ICFR` writer"]
pub type W = crate::W<Ch3icfrSpec>;
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
    pub fn ti3f(&mut self) -> Ti3fW<'_, Ch3icfrSpec> {
        Ti3fW::new(self, 0)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    pub fn ch3ccs(&mut self) -> Ch3ccsW<'_, Ch3icfrSpec> {
        Ch3ccsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    pub fn ch3psc(&mut self) -> Ch3pscW<'_, Ch3icfrSpec> {
        Ch3pscW::new(self, 18)
    }
}
#[doc = "CH3ICFR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3icfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3icfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3icfrSpec;
impl crate::RegisterSpec for Ch3icfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3icfr::R`](R) reader structure"]
impl crate::Readable for Ch3icfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3icfr::W`](W) writer structure"]
impl crate::Writable for Ch3icfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3ICFR to value 0"]
impl crate::Resettable for Ch3icfrSpec {}
