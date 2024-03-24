#[doc = "Register `CHPOLR` reader"]
pub type R = crate::R<ChpolrSpec>;
#[doc = "Register `CHPOLR` writer"]
pub type W = crate::W<ChpolrSpec>;
#[doc = "Field `CH0P` reader - CH0P"]
pub type Ch0pR = crate::BitReader;
#[doc = "Field `CH0P` writer - CH0P"]
pub type Ch0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0NP` reader - CH0NP"]
pub type Ch0npR = crate::BitReader;
#[doc = "Field `CH0NP` writer - CH0NP"]
pub type Ch0npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1P` reader - CH1P"]
pub type Ch1pR = crate::BitReader;
#[doc = "Field `CH1P` writer - CH1P"]
pub type Ch1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1NP` reader - CH1NP"]
pub type Ch1npR = crate::BitReader;
#[doc = "Field `CH1NP` writer - CH1NP"]
pub type Ch1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2P` reader - CH2P"]
pub type Ch2pR = crate::BitReader;
#[doc = "Field `CH2P` writer - CH2P"]
pub type Ch2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2NP` reader - CH2NP"]
pub type Ch2npR = crate::BitReader;
#[doc = "Field `CH2NP` writer - CH2NP"]
pub type Ch2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3P` reader - CH3P"]
pub type Ch3pR = crate::BitReader;
#[doc = "Field `CH3P` writer - CH3P"]
pub type Ch3pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    pub fn ch0p(&self) -> Ch0pR {
        Ch0pR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    pub fn ch0np(&self) -> Ch0npR {
        Ch0npR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&self) -> Ch1pR {
        Ch1pR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    pub fn ch1np(&self) -> Ch1npR {
        Ch1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&self) -> Ch2pR {
        Ch2pR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    pub fn ch2np(&self) -> Ch2npR {
        Ch2npR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&self) -> Ch3pR {
        Ch3pR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> Ch0pW<ChpolrSpec> {
        Ch0pW::new(self, 0)
    }
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> Ch0npW<ChpolrSpec> {
        Ch0npW::new(self, 1)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> Ch1pW<ChpolrSpec> {
        Ch1pW::new(self, 2)
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> Ch1npW<ChpolrSpec> {
        Ch1npW::new(self, 3)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> Ch2pW<ChpolrSpec> {
        Ch2pW::new(self, 4)
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch2np(&mut self) -> Ch2npW<ChpolrSpec> {
        Ch2npW::new(self, 5)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> Ch3pW<ChpolrSpec> {
        Ch3pW::new(self, 6)
    }
}
#[doc = "CHPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpolr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpolr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChpolrSpec;
impl crate::RegisterSpec for ChpolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpolr::R`](R) reader structure"]
impl crate::Readable for ChpolrSpec {}
#[doc = "`write(|w| ..)` method takes [`chpolr::W`](W) writer structure"]
impl crate::Writable for ChpolrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHPOLR to value 0"]
impl crate::Resettable for ChpolrSpec {
    const RESET_VALUE: u32 = 0;
}
