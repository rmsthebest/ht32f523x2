#[doc = "Register `GPTM_INTSR` reader"]
pub type R = crate::R<GptmIntsrSpec>;
#[doc = "Register `GPTM_INTSR` writer"]
pub type W = crate::W<GptmIntsrSpec>;
#[doc = "Field `CH0CCIF` reader - CH0CCIF"]
pub type Ch0ccifR = crate::BitReader;
#[doc = "Field `CH0CCIF` writer - CH0CCIF"]
pub type Ch0ccifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CCIF` reader - CH1CCIF"]
pub type Ch1ccifR = crate::BitReader;
#[doc = "Field `CH1CCIF` writer - CH1CCIF"]
pub type Ch1ccifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CCIF` reader - CH2CCIF"]
pub type Ch2ccifR = crate::BitReader;
#[doc = "Field `CH2CCIF` writer - CH2CCIF"]
pub type Ch2ccifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CCIF` reader - CH3CCIF"]
pub type Ch3ccifR = crate::BitReader;
#[doc = "Field `CH3CCIF` writer - CH3CCIF"]
pub type Ch3ccifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OCF` reader - CH0OCF"]
pub type Ch0ocfR = crate::BitReader;
#[doc = "Field `CH0OCF` writer - CH0OCF"]
pub type Ch0ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OCF` reader - CH1OCF"]
pub type Ch1ocfR = crate::BitReader;
#[doc = "Field `CH1OCF` writer - CH1OCF"]
pub type Ch1ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OCF` reader - CH2OCF"]
pub type Ch2ocfR = crate::BitReader;
#[doc = "Field `CH2OCF` writer - CH2OCF"]
pub type Ch2ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OCF` reader - CH3OCF"]
pub type Ch3ocfR = crate::BitReader;
#[doc = "Field `CH3OCF` writer - CH3OCF"]
pub type Ch3ocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEVIF` reader - UEVIF"]
pub type UevifR = crate::BitReader;
#[doc = "Field `UEVIF` writer - UEVIF"]
pub type UevifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEVIF` reader - TEVIF"]
pub type TevifR = crate::BitReader;
#[doc = "Field `TEVIF` writer - TEVIF"]
pub type TevifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&self) -> Ch0ccifR {
        Ch0ccifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    pub fn ch1ccif(&self) -> Ch1ccifR {
        Ch1ccifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    pub fn ch2ccif(&self) -> Ch2ccifR {
        Ch2ccifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    pub fn ch3ccif(&self) -> Ch3ccifR {
        Ch3ccifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&self) -> Ch0ocfR {
        Ch0ocfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    pub fn ch1ocf(&self) -> Ch1ocfR {
        Ch1ocfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    pub fn ch2ocf(&self) -> Ch2ocfR {
        Ch2ocfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    pub fn ch3ocf(&self) -> Ch3ocfR {
        Ch3ocfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&self) -> UevifR {
        UevifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TevifR {
        TevifR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccif(&mut self) -> Ch0ccifW<GptmIntsrSpec> {
        Ch0ccifW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccif(&mut self) -> Ch1ccifW<GptmIntsrSpec> {
        Ch1ccifW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccif(&mut self) -> Ch2ccifW<GptmIntsrSpec> {
        Ch2ccifW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccif(&mut self) -> Ch3ccifW<GptmIntsrSpec> {
        Ch3ccifW::new(self, 3)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ocf(&mut self) -> Ch0ocfW<GptmIntsrSpec> {
        Ch0ocfW::new(self, 4)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ocf(&mut self) -> Ch1ocfW<GptmIntsrSpec> {
        Ch1ocfW::new(self, 5)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ocf(&mut self) -> Ch2ocfW<GptmIntsrSpec> {
        Ch2ocfW::new(self, 6)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ocf(&mut self) -> Ch3ocfW<GptmIntsrSpec> {
        Ch3ocfW::new(self, 7)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn uevif(&mut self) -> UevifW<GptmIntsrSpec> {
        UevifW::new(self, 8)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn tevif(&mut self) -> TevifW<GptmIntsrSpec> {
        TevifW::new(self, 10)
    }
}
#[doc = "GPTM_INTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_intsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_intsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmIntsrSpec;
impl crate::RegisterSpec for GptmIntsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_intsr::R`](R) reader structure"]
impl crate::Readable for GptmIntsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_intsr::W`](W) writer structure"]
impl crate::Writable for GptmIntsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_INTSR to value 0"]
impl crate::Resettable for GptmIntsrSpec {
    const RESET_VALUE: u32 = 0;
}
