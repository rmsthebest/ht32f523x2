#[doc = "Register `CHBRKCFR` reader"]
pub type R = crate::R<ChbrkcfrSpec>;
#[doc = "Register `CHBRKCFR` writer"]
pub type W = crate::W<ChbrkcfrSpec>;
#[doc = "Field `CH0OIS` reader - CH0OIS"]
pub type Ch0oisR = crate::BitReader;
#[doc = "Field `CH0OIS` writer - CH0OIS"]
pub type Ch0oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OISN` reader - CH0OISN"]
pub type Ch0oisnR = crate::BitReader;
#[doc = "Field `CH0OISN` writer - CH0OISN"]
pub type Ch0oisnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OIS` reader - CH1OIS"]
pub type Ch1oisR = crate::BitReader;
#[doc = "Field `CH1OIS` writer - CH1OIS"]
pub type Ch1oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OISN` reader - CH1OISN"]
pub type Ch1oisnR = crate::BitReader;
#[doc = "Field `CH1OISN` writer - CH1OISN"]
pub type Ch1oisnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OIS` reader - CH2OIS"]
pub type Ch2oisR = crate::BitReader;
#[doc = "Field `CH2OIS` writer - CH2OIS"]
pub type Ch2oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OISN` reader - CH2OISN"]
pub type Ch2oisnR = crate::BitReader;
#[doc = "Field `CH2OISN` writer - CH2OISN"]
pub type Ch2oisnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OIS` reader - CH3OIS"]
pub type Ch3oisR = crate::BitReader;
#[doc = "Field `CH3OIS` writer - CH3OIS"]
pub type Ch3oisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    pub fn ch0ois(&self) -> Ch0oisR {
        Ch0oisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    pub fn ch0oisn(&self) -> Ch0oisnR {
        Ch0oisnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    pub fn ch1ois(&self) -> Ch1oisR {
        Ch1oisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    pub fn ch1oisn(&self) -> Ch1oisnR {
        Ch1oisnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    pub fn ch2ois(&self) -> Ch2oisR {
        Ch2oisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    pub fn ch2oisn(&self) -> Ch2oisnR {
        Ch2oisnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    pub fn ch3ois(&self) -> Ch3oisR {
        Ch3oisR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ois(&mut self) -> Ch0oisW<ChbrkcfrSpec> {
        Ch0oisW::new(self, 0)
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oisn(&mut self) -> Ch0oisnW<ChbrkcfrSpec> {
        Ch0oisnW::new(self, 1)
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ois(&mut self) -> Ch1oisW<ChbrkcfrSpec> {
        Ch1oisW::new(self, 2)
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oisn(&mut self) -> Ch1oisnW<ChbrkcfrSpec> {
        Ch1oisnW::new(self, 3)
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ois(&mut self) -> Ch2oisW<ChbrkcfrSpec> {
        Ch2oisW::new(self, 4)
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch2oisn(&mut self) -> Ch2oisnW<ChbrkcfrSpec> {
        Ch2oisnW::new(self, 5)
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ois(&mut self) -> Ch3oisW<ChbrkcfrSpec> {
        Ch3oisW::new(self, 6)
    }
}
#[doc = "CHBRKCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbrkcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chbrkcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChbrkcfrSpec;
impl crate::RegisterSpec for ChbrkcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chbrkcfr::R`](R) reader structure"]
impl crate::Readable for ChbrkcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`chbrkcfr::W`](W) writer structure"]
impl crate::Writable for ChbrkcfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHBRKCFR to value 0"]
impl crate::Resettable for ChbrkcfrSpec {
    const RESET_VALUE: u32 = 0;
}
