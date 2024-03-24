#[doc = "Register `CHCTR` reader"]
pub type R = crate::R<ChctrSpec>;
#[doc = "Register `CHCTR` writer"]
pub type W = crate::W<ChctrSpec>;
#[doc = "Field `CH0E` reader - CH0E"]
pub type Ch0eR = crate::BitReader;
#[doc = "Field `CH0E` writer - CH0E"]
pub type Ch0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0NE` reader - CH0NE"]
pub type Ch0neR = crate::BitReader;
#[doc = "Field `CH0NE` writer - CH0NE"]
pub type Ch0neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1E` reader - CH1E"]
pub type Ch1eR = crate::BitReader;
#[doc = "Field `CH1E` writer - CH1E"]
pub type Ch1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1NE` reader - CH1NE"]
pub type Ch1neR = crate::BitReader;
#[doc = "Field `CH1NE` writer - CH1NE"]
pub type Ch1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2E` reader - CH2E"]
pub type Ch2eR = crate::BitReader;
#[doc = "Field `CH2E` writer - CH2E"]
pub type Ch2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2NE` reader - CH2NE"]
pub type Ch2neR = crate::BitReader;
#[doc = "Field `CH2NE` writer - CH2NE"]
pub type Ch2neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3E` reader - CH3E"]
pub type Ch3eR = crate::BitReader;
#[doc = "Field `CH3E` writer - CH3E"]
pub type Ch3eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&self) -> Ch0eR {
        Ch0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    pub fn ch0ne(&self) -> Ch0neR {
        Ch0neR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&self) -> Ch1eR {
        Ch1eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    pub fn ch1ne(&self) -> Ch1neR {
        Ch1neR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&self) -> Ch2eR {
        Ch2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    pub fn ch2ne(&self) -> Ch2neR {
        Ch2neR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&self) -> Ch3eR {
        Ch3eR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    #[must_use]
    pub fn ch0e(&mut self) -> Ch0eW<ChctrSpec> {
        Ch0eW::new(self, 0)
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ne(&mut self) -> Ch0neW<ChctrSpec> {
        Ch0neW::new(self, 1)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    #[must_use]
    pub fn ch1e(&mut self) -> Ch1eW<ChctrSpec> {
        Ch1eW::new(self, 2)
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ne(&mut self) -> Ch1neW<ChctrSpec> {
        Ch1neW::new(self, 3)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    #[must_use]
    pub fn ch2e(&mut self) -> Ch2eW<ChctrSpec> {
        Ch2eW::new(self, 4)
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ne(&mut self) -> Ch2neW<ChctrSpec> {
        Ch2neW::new(self, 5)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    #[must_use]
    pub fn ch3e(&mut self) -> Ch3eW<ChctrSpec> {
        Ch3eW::new(self, 6)
    }
}
#[doc = "CHCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrSpec;
impl crate::RegisterSpec for ChctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctr::R`](R) reader structure"]
impl crate::Readable for ChctrSpec {}
#[doc = "`write(|w| ..)` method takes [`chctr::W`](W) writer structure"]
impl crate::Writable for ChctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTR to value 0"]
impl crate::Resettable for ChctrSpec {
    const RESET_VALUE: u32 = 0;
}
