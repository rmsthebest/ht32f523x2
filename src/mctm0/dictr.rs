#[doc = "Register `DICTR` reader"]
pub type R = crate::R<DictrSpec>;
#[doc = "Register `DICTR` writer"]
pub type W = crate::W<DictrSpec>;
#[doc = "Field `CH0CCIE` reader - CH0CCIE"]
pub type Ch0ccieR = crate::BitReader;
#[doc = "Field `CH0CCIE` writer - CH0CCIE"]
pub type Ch0ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCIE` reader - CH1CCIE"]
pub type Ch1ccieR = crate::BitReader;
#[doc = "Field `CH1CCIE` writer - CH1CCIE"]
pub type Ch1ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCIE` reader - CH2CCIE"]
pub type Ch2ccieR = crate::BitReader;
#[doc = "Field `CH2CCIE` writer - CH2CCIE"]
pub type Ch2ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCIE` reader - CH3CCIE"]
pub type Ch3ccieR = crate::BitReader;
#[doc = "Field `CH3CCIE` writer - CH3CCIE"]
pub type Ch3ccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEV1IE` reader - UEV1IE"]
pub type Uev1ieR = crate::BitReader;
#[doc = "Field `UEV1IE` writer - UEV1IE"]
pub type Uev1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEV2IE` reader - UEV2IE"]
pub type Uev2ieR = crate::BitReader;
#[doc = "Field `UEV2IE` writer - UEV2IE"]
pub type Uev2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TevieR = crate::BitReader;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TevieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIE` reader - BRKIE"]
pub type BrkieR = crate::BitReader;
#[doc = "Field `BRKIE` writer - BRKIE"]
pub type BrkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CCDE` reader - CH0CCDE"]
pub type Ch0ccdeR = crate::BitReader;
#[doc = "Field `CH0CCDE` writer - CH0CCDE"]
pub type Ch0ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCDE` reader - CH1CCDE"]
pub type Ch1ccdeR = crate::BitReader;
#[doc = "Field `CH1CCDE` writer - CH1CCDE"]
pub type Ch1ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCDE` reader - CH2CCDE"]
pub type Ch2ccdeR = crate::BitReader;
#[doc = "Field `CH2CCDE` writer - CH2CCDE"]
pub type Ch2ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCDE` reader - CH3CCDE"]
pub type Ch3ccdeR = crate::BitReader;
#[doc = "Field `CH3CCDE` writer - CH3CCDE"]
pub type Ch3ccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEV1DE` reader - UEV1DE"]
pub type Uev1deR = crate::BitReader;
#[doc = "Field `UEV1DE` writer - UEV1DE"]
pub type Uev1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEV2DE` reader - UEV2DE"]
pub type Uev2deR = crate::BitReader;
#[doc = "Field `UEV2DE` writer - UEV2DE"]
pub type Uev2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVDE` reader - TEVDE"]
pub type TevdeR = crate::BitReader;
#[doc = "Field `TEVDE` writer - TEVDE"]
pub type TevdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> Ch0ccieR {
        Ch0ccieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> Ch1ccieR {
        Ch1ccieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&self) -> Ch2ccieR {
        Ch2ccieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&self) -> Ch3ccieR {
        Ch3ccieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    pub fn uev1ie(&self) -> Uev1ieR {
        Uev1ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    pub fn uev2ie(&self) -> Uev2ieR {
        Uev2ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TevieR {
        TevieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    pub fn brkie(&self) -> BrkieR {
        BrkieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&self) -> Ch0ccdeR {
        Ch0ccdeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&self) -> Ch1ccdeR {
        Ch1ccdeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    pub fn ch2ccde(&self) -> Ch2ccdeR {
        Ch2ccdeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    pub fn ch3ccde(&self) -> Ch3ccdeR {
        Ch3ccdeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    pub fn uev1de(&self) -> Uev1deR {
        Uev1deR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    pub fn uev2de(&self) -> Uev2deR {
        Uev2deR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    pub fn tevde(&self) -> TevdeR {
        TevdeR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccie(&mut self) -> Ch0ccieW<DictrSpec> {
        Ch0ccieW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccie(&mut self) -> Ch1ccieW<DictrSpec> {
        Ch1ccieW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccie(&mut self) -> Ch2ccieW<DictrSpec> {
        Ch2ccieW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccie(&mut self) -> Ch3ccieW<DictrSpec> {
        Ch3ccieW::new(self, 3)
    }
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    #[must_use]
    pub fn uev1ie(&mut self) -> Uev1ieW<DictrSpec> {
        Uev1ieW::new(self, 8)
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    #[must_use]
    pub fn uev2ie(&mut self) -> Uev2ieW<DictrSpec> {
        Uev2ieW::new(self, 9)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn tevie(&mut self) -> TevieW<DictrSpec> {
        TevieW::new(self, 10)
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BrkieW<DictrSpec> {
        BrkieW::new(self, 11)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccde(&mut self) -> Ch0ccdeW<DictrSpec> {
        Ch0ccdeW::new(self, 16)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccde(&mut self) -> Ch1ccdeW<DictrSpec> {
        Ch1ccdeW::new(self, 17)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccde(&mut self) -> Ch2ccdeW<DictrSpec> {
        Ch2ccdeW::new(self, 18)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccde(&mut self) -> Ch3ccdeW<DictrSpec> {
        Ch3ccdeW::new(self, 19)
    }
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    #[must_use]
    pub fn uev1de(&mut self) -> Uev1deW<DictrSpec> {
        Uev1deW::new(self, 24)
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    #[must_use]
    pub fn uev2de(&mut self) -> Uev2deW<DictrSpec> {
        Uev2deW::new(self, 25)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    #[must_use]
    pub fn tevde(&mut self) -> TevdeW<DictrSpec> {
        TevdeW::new(self, 26)
    }
}
#[doc = "DICTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dictr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dictr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DictrSpec;
impl crate::RegisterSpec for DictrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dictr::R`](R) reader structure"]
impl crate::Readable for DictrSpec {}
#[doc = "`write(|w| ..)` method takes [`dictr::W`](W) writer structure"]
impl crate::Writable for DictrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DICTR to value 0"]
impl crate::Resettable for DictrSpec {
    const RESET_VALUE: u32 = 0;
}
