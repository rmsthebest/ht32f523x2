#[doc = "Register `EDGEFLGR` reader"]
pub type R = crate::R<EdgeflgrSpec>;
#[doc = "Register `EDGEFLGR` writer"]
pub type W = crate::W<EdgeflgrSpec>;
#[doc = "Field `EXTI0EDF` reader - EXTI0EDF"]
pub type Exti0edfR = crate::BitReader;
#[doc = "Field `EXTI0EDF` writer - EXTI0EDF"]
pub type Exti0edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1EDF` reader - EXTI1EDF"]
pub type Exti1edfR = crate::BitReader;
#[doc = "Field `EXTI1EDF` writer - EXTI1EDF"]
pub type Exti1edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2EDF` reader - EXTI2EDF"]
pub type Exti2edfR = crate::BitReader;
#[doc = "Field `EXTI2EDF` writer - EXTI2EDF"]
pub type Exti2edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3EDF` reader - EXTI3EDF"]
pub type Exti3edfR = crate::BitReader;
#[doc = "Field `EXTI3EDF` writer - EXTI3EDF"]
pub type Exti3edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4EDF` reader - EXTI4EDF"]
pub type Exti4edfR = crate::BitReader;
#[doc = "Field `EXTI4EDF` writer - EXTI4EDF"]
pub type Exti4edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5EDF` reader - EXTI5EDF"]
pub type Exti5edfR = crate::BitReader;
#[doc = "Field `EXTI5EDF` writer - EXTI5EDF"]
pub type Exti5edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6EDF` reader - EXTI6EDF"]
pub type Exti6edfR = crate::BitReader;
#[doc = "Field `EXTI6EDF` writer - EXTI6EDF"]
pub type Exti6edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7EDF` reader - EXTI7EDF"]
pub type Exti7edfR = crate::BitReader;
#[doc = "Field `EXTI7EDF` writer - EXTI7EDF"]
pub type Exti7edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8EDF` reader - EXTI8EDF"]
pub type Exti8edfR = crate::BitReader;
#[doc = "Field `EXTI8EDF` writer - EXTI8EDF"]
pub type Exti8edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9EDF` reader - EXTI9EDF"]
pub type Exti9edfR = crate::BitReader;
#[doc = "Field `EXTI9EDF` writer - EXTI9EDF"]
pub type Exti9edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10EDF` reader - EXTI10EDF"]
pub type Exti10edfR = crate::BitReader;
#[doc = "Field `EXTI10EDF` writer - EXTI10EDF"]
pub type Exti10edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11EDF` reader - EXTI11EDF"]
pub type Exti11edfR = crate::BitReader;
#[doc = "Field `EXTI11EDF` writer - EXTI11EDF"]
pub type Exti11edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12EDF` reader - EXTI12EDF"]
pub type Exti12edfR = crate::BitReader;
#[doc = "Field `EXTI12EDF` writer - EXTI12EDF"]
pub type Exti12edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13EDF` reader - EXTI13EDF"]
pub type Exti13edfR = crate::BitReader;
#[doc = "Field `EXTI13EDF` writer - EXTI13EDF"]
pub type Exti13edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14EDF` reader - EXTI14EDF"]
pub type Exti14edfR = crate::BitReader;
#[doc = "Field `EXTI14EDF` writer - EXTI14EDF"]
pub type Exti14edfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15EDF` reader - EXTI15EDF"]
pub type Exti15edfR = crate::BitReader;
#[doc = "Field `EXTI15EDF` writer - EXTI15EDF"]
pub type Exti15edfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    pub fn exti0edf(&self) -> Exti0edfR {
        Exti0edfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    pub fn exti1edf(&self) -> Exti1edfR {
        Exti1edfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    pub fn exti2edf(&self) -> Exti2edfR {
        Exti2edfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    pub fn exti3edf(&self) -> Exti3edfR {
        Exti3edfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    pub fn exti4edf(&self) -> Exti4edfR {
        Exti4edfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    pub fn exti5edf(&self) -> Exti5edfR {
        Exti5edfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    pub fn exti6edf(&self) -> Exti6edfR {
        Exti6edfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    pub fn exti7edf(&self) -> Exti7edfR {
        Exti7edfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    pub fn exti8edf(&self) -> Exti8edfR {
        Exti8edfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    pub fn exti9edf(&self) -> Exti9edfR {
        Exti9edfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    pub fn exti10edf(&self) -> Exti10edfR {
        Exti10edfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    pub fn exti11edf(&self) -> Exti11edfR {
        Exti11edfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    pub fn exti12edf(&self) -> Exti12edfR {
        Exti12edfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    pub fn exti13edf(&self) -> Exti13edfR {
        Exti13edfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    pub fn exti14edf(&self) -> Exti14edfR {
        Exti14edfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    pub fn exti15edf(&self) -> Exti15edfR {
        Exti15edfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti0edf(&mut self) -> Exti0edfW<EdgeflgrSpec> {
        Exti0edfW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti1edf(&mut self) -> Exti1edfW<EdgeflgrSpec> {
        Exti1edfW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti2edf(&mut self) -> Exti2edfW<EdgeflgrSpec> {
        Exti2edfW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti3edf(&mut self) -> Exti3edfW<EdgeflgrSpec> {
        Exti3edfW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti4edf(&mut self) -> Exti4edfW<EdgeflgrSpec> {
        Exti4edfW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti5edf(&mut self) -> Exti5edfW<EdgeflgrSpec> {
        Exti5edfW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti6edf(&mut self) -> Exti6edfW<EdgeflgrSpec> {
        Exti6edfW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti7edf(&mut self) -> Exti7edfW<EdgeflgrSpec> {
        Exti7edfW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti8edf(&mut self) -> Exti8edfW<EdgeflgrSpec> {
        Exti8edfW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti9edf(&mut self) -> Exti9edfW<EdgeflgrSpec> {
        Exti9edfW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti10edf(&mut self) -> Exti10edfW<EdgeflgrSpec> {
        Exti10edfW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti11edf(&mut self) -> Exti11edfW<EdgeflgrSpec> {
        Exti11edfW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti12edf(&mut self) -> Exti12edfW<EdgeflgrSpec> {
        Exti12edfW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti13edf(&mut self) -> Exti13edfW<EdgeflgrSpec> {
        Exti13edfW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti14edf(&mut self) -> Exti14edfW<EdgeflgrSpec> {
        Exti14edfW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    #[must_use]
    pub fn exti15edf(&mut self) -> Exti15edfW<EdgeflgrSpec> {
        Exti15edfW::new(self, 15)
    }
}
#[doc = "EDGEFLGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edgeflgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edgeflgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdgeflgrSpec;
impl crate::RegisterSpec for EdgeflgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edgeflgr::R`](R) reader structure"]
impl crate::Readable for EdgeflgrSpec {}
#[doc = "`write(|w| ..)` method takes [`edgeflgr::W`](W) writer structure"]
impl crate::Writable for EdgeflgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDGEFLGR to value 0"]
impl crate::Resettable for EdgeflgrSpec {
    const RESET_VALUE: u32 = 0;
}
