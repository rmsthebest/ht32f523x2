#[doc = "Register `CH2ICFR` reader"]
pub type R = crate::R<Ch2icfrSpec>;
#[doc = "Register `CH2ICFR` writer"]
pub type W = crate::W<Ch2icfrSpec>;
#[doc = "Field `TI2F` reader - TI2F"]
pub type Ti2fR = crate::FieldReader;
#[doc = "Field `TI2F` writer - TI2F"]
pub type Ti2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2CCS` reader - CH2CCS"]
pub type Ch2ccsR = crate::FieldReader;
#[doc = "Field `CH2CCS` writer - CH2CCS"]
pub type Ch2ccsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2PSC` reader - CH2PSC"]
pub type Ch2pscR = crate::FieldReader;
#[doc = "Field `CH2PSC` writer - CH2PSC"]
pub type Ch2pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    pub fn ti2f(&self) -> Ti2fR {
        Ti2fR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    pub fn ch2ccs(&self) -> Ch2ccsR {
        Ch2ccsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    pub fn ch2psc(&self) -> Ch2pscR {
        Ch2pscR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    #[must_use]
    pub fn ti2f(&mut self) -> Ti2fW<Ch2icfrSpec> {
        Ti2fW::new(self, 0)
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccs(&mut self) -> Ch2ccsW<Ch2icfrSpec> {
        Ch2ccsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch2psc(&mut self) -> Ch2pscW<Ch2icfrSpec> {
        Ch2pscW::new(self, 18)
    }
}
#[doc = "CH2ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2icfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2icfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2icfrSpec;
impl crate::RegisterSpec for Ch2icfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2icfr::R`](R) reader structure"]
impl crate::Readable for Ch2icfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2icfr::W`](W) writer structure"]
impl crate::Writable for Ch2icfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2ICFR to value 0"]
impl crate::Resettable for Ch2icfrSpec {
    const RESET_VALUE: u32 = 0;
}
