#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EXTI0EN` reader - EXTI0EN"]
pub type Exti0enR = crate::BitReader;
#[doc = "Field `EXTI0EN` writer - EXTI0EN"]
pub type Exti0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1EN` reader - EXTI1EN"]
pub type Exti1enR = crate::BitReader;
#[doc = "Field `EXTI1EN` writer - EXTI1EN"]
pub type Exti1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2EN` reader - EXTI2EN"]
pub type Exti2enR = crate::BitReader;
#[doc = "Field `EXTI2EN` writer - EXTI2EN"]
pub type Exti2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3EN` reader - EXTI3EN"]
pub type Exti3enR = crate::BitReader;
#[doc = "Field `EXTI3EN` writer - EXTI3EN"]
pub type Exti3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4EN` reader - EXTI4EN"]
pub type Exti4enR = crate::BitReader;
#[doc = "Field `EXTI4EN` writer - EXTI4EN"]
pub type Exti4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5EN` reader - EXTI5EN"]
pub type Exti5enR = crate::BitReader;
#[doc = "Field `EXTI5EN` writer - EXTI5EN"]
pub type Exti5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6EN` reader - EXTI6EN"]
pub type Exti6enR = crate::BitReader;
#[doc = "Field `EXTI6EN` writer - EXTI6EN"]
pub type Exti6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7EN` reader - EXTI7EN"]
pub type Exti7enR = crate::BitReader;
#[doc = "Field `EXTI7EN` writer - EXTI7EN"]
pub type Exti7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8EN` reader - EXTI8EN"]
pub type Exti8enR = crate::BitReader;
#[doc = "Field `EXTI8EN` writer - EXTI8EN"]
pub type Exti8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9EN` reader - EXTI9EN"]
pub type Exti9enR = crate::BitReader;
#[doc = "Field `EXTI9EN` writer - EXTI9EN"]
pub type Exti9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10EN` reader - EXTI10EN"]
pub type Exti10enR = crate::BitReader;
#[doc = "Field `EXTI10EN` writer - EXTI10EN"]
pub type Exti10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11EN` reader - EXTI11EN"]
pub type Exti11enR = crate::BitReader;
#[doc = "Field `EXTI11EN` writer - EXTI11EN"]
pub type Exti11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12EN` reader - EXTI12EN"]
pub type Exti12enR = crate::BitReader;
#[doc = "Field `EXTI12EN` writer - EXTI12EN"]
pub type Exti12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13EN` reader - EXTI13EN"]
pub type Exti13enR = crate::BitReader;
#[doc = "Field `EXTI13EN` writer - EXTI13EN"]
pub type Exti13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14EN` reader - EXTI14EN"]
pub type Exti14enR = crate::BitReader;
#[doc = "Field `EXTI14EN` writer - EXTI14EN"]
pub type Exti14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15EN` reader - EXTI15EN"]
pub type Exti15enR = crate::BitReader;
#[doc = "Field `EXTI15EN` writer - EXTI15EN"]
pub type Exti15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    pub fn exti0en(&self) -> Exti0enR {
        Exti0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    pub fn exti1en(&self) -> Exti1enR {
        Exti1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    pub fn exti2en(&self) -> Exti2enR {
        Exti2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    pub fn exti3en(&self) -> Exti3enR {
        Exti3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    pub fn exti4en(&self) -> Exti4enR {
        Exti4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    pub fn exti5en(&self) -> Exti5enR {
        Exti5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    pub fn exti6en(&self) -> Exti6enR {
        Exti6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    pub fn exti7en(&self) -> Exti7enR {
        Exti7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    pub fn exti8en(&self) -> Exti8enR {
        Exti8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    pub fn exti9en(&self) -> Exti9enR {
        Exti9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    pub fn exti10en(&self) -> Exti10enR {
        Exti10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    pub fn exti11en(&self) -> Exti11enR {
        Exti11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    pub fn exti12en(&self) -> Exti12enR {
        Exti12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    pub fn exti13en(&self) -> Exti13enR {
        Exti13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    pub fn exti14en(&self) -> Exti14enR {
        Exti14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    pub fn exti15en(&self) -> Exti15enR {
        Exti15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0en(&mut self) -> Exti0enW<CrSpec> {
        Exti0enW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1en(&mut self) -> Exti1enW<CrSpec> {
        Exti1enW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2en(&mut self) -> Exti2enW<CrSpec> {
        Exti2enW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3en(&mut self) -> Exti3enW<CrSpec> {
        Exti3enW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4en(&mut self) -> Exti4enW<CrSpec> {
        Exti4enW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5en(&mut self) -> Exti5enW<CrSpec> {
        Exti5enW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6en(&mut self) -> Exti6enW<CrSpec> {
        Exti6enW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7en(&mut self) -> Exti7enW<CrSpec> {
        Exti7enW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8en(&mut self) -> Exti8enW<CrSpec> {
        Exti8enW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9en(&mut self) -> Exti9enW<CrSpec> {
        Exti9enW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10en(&mut self) -> Exti10enW<CrSpec> {
        Exti10enW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11en(&mut self) -> Exti11enW<CrSpec> {
        Exti11enW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12en(&mut self) -> Exti12enW<CrSpec> {
        Exti12enW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13en(&mut self) -> Exti13enW<CrSpec> {
        Exti13enW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14en(&mut self) -> Exti14enW<CrSpec> {
        Exti14enW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15en(&mut self) -> Exti15enW<CrSpec> {
        Exti15enW::new(self, 15)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
