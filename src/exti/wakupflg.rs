#[doc = "Register `WAKUPFLG` reader"]
pub type R = crate::R<WakupflgSpec>;
#[doc = "Register `WAKUPFLG` writer"]
pub type W = crate::W<WakupflgSpec>;
#[doc = "Field `EXTI0WFL` reader - EXTI0WFL"]
pub type Exti0wflR = crate::BitReader;
#[doc = "Field `EXTI0WFL` writer - EXTI0WFL"]
pub type Exti0wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1WFL` reader - EXTI1WFL"]
pub type Exti1wflR = crate::BitReader;
#[doc = "Field `EXTI1WFL` writer - EXTI1WFL"]
pub type Exti1wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2WFL` reader - EXTI2WFL"]
pub type Exti2wflR = crate::BitReader;
#[doc = "Field `EXTI2WFL` writer - EXTI2WFL"]
pub type Exti2wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3WFL` reader - EXTI3WFL"]
pub type Exti3wflR = crate::BitReader;
#[doc = "Field `EXTI3WFL` writer - EXTI3WFL"]
pub type Exti3wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4WFL` reader - EXTI4WFL"]
pub type Exti4wflR = crate::BitReader;
#[doc = "Field `EXTI4WFL` writer - EXTI4WFL"]
pub type Exti4wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5WFL` reader - EXTI5WFL"]
pub type Exti5wflR = crate::BitReader;
#[doc = "Field `EXTI5WFL` writer - EXTI5WFL"]
pub type Exti5wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6WFL` reader - EXTI6WFL"]
pub type Exti6wflR = crate::BitReader;
#[doc = "Field `EXTI6WFL` writer - EXTI6WFL"]
pub type Exti6wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7WFL` reader - EXTI7WFL"]
pub type Exti7wflR = crate::BitReader;
#[doc = "Field `EXTI7WFL` writer - EXTI7WFL"]
pub type Exti7wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8WFL` reader - EXTI8WFL"]
pub type Exti8wflR = crate::BitReader;
#[doc = "Field `EXTI8WFL` writer - EXTI8WFL"]
pub type Exti8wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9WFL` reader - EXTI9WFL"]
pub type Exti9wflR = crate::BitReader;
#[doc = "Field `EXTI9WFL` writer - EXTI9WFL"]
pub type Exti9wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10WFL` reader - EXTI10WFL"]
pub type Exti10wflR = crate::BitReader;
#[doc = "Field `EXTI10WFL` writer - EXTI10WFL"]
pub type Exti10wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11WFL` reader - EXTI11WFL"]
pub type Exti11wflR = crate::BitReader;
#[doc = "Field `EXTI11WFL` writer - EXTI11WFL"]
pub type Exti11wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12WFL` reader - EXTI12WFL"]
pub type Exti12wflR = crate::BitReader;
#[doc = "Field `EXTI12WFL` writer - EXTI12WFL"]
pub type Exti12wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13WFL` reader - EXTI13WFL"]
pub type Exti13wflR = crate::BitReader;
#[doc = "Field `EXTI13WFL` writer - EXTI13WFL"]
pub type Exti13wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14WFL` reader - EXTI14WFL"]
pub type Exti14wflR = crate::BitReader;
#[doc = "Field `EXTI14WFL` writer - EXTI14WFL"]
pub type Exti14wflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15WFL` reader - EXTI15WFL"]
pub type Exti15wflR = crate::BitReader;
#[doc = "Field `EXTI15WFL` writer - EXTI15WFL"]
pub type Exti15wflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    pub fn exti0wfl(&self) -> Exti0wflR {
        Exti0wflR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    pub fn exti1wfl(&self) -> Exti1wflR {
        Exti1wflR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    pub fn exti2wfl(&self) -> Exti2wflR {
        Exti2wflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    pub fn exti3wfl(&self) -> Exti3wflR {
        Exti3wflR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    pub fn exti4wfl(&self) -> Exti4wflR {
        Exti4wflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    pub fn exti5wfl(&self) -> Exti5wflR {
        Exti5wflR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    pub fn exti6wfl(&self) -> Exti6wflR {
        Exti6wflR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    pub fn exti7wfl(&self) -> Exti7wflR {
        Exti7wflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    pub fn exti8wfl(&self) -> Exti8wflR {
        Exti8wflR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    pub fn exti9wfl(&self) -> Exti9wflR {
        Exti9wflR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    pub fn exti10wfl(&self) -> Exti10wflR {
        Exti10wflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    pub fn exti11wfl(&self) -> Exti11wflR {
        Exti11wflR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    pub fn exti12wfl(&self) -> Exti12wflR {
        Exti12wflR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    pub fn exti13wfl(&self) -> Exti13wflR {
        Exti13wflR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    pub fn exti14wfl(&self) -> Exti14wflR {
        Exti14wflR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    pub fn exti15wfl(&self) -> Exti15wflR {
        Exti15wflR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wfl(&mut self) -> Exti0wflW<WakupflgSpec> {
        Exti0wflW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wfl(&mut self) -> Exti1wflW<WakupflgSpec> {
        Exti1wflW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wfl(&mut self) -> Exti2wflW<WakupflgSpec> {
        Exti2wflW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wfl(&mut self) -> Exti3wflW<WakupflgSpec> {
        Exti3wflW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wfl(&mut self) -> Exti4wflW<WakupflgSpec> {
        Exti4wflW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wfl(&mut self) -> Exti5wflW<WakupflgSpec> {
        Exti5wflW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wfl(&mut self) -> Exti6wflW<WakupflgSpec> {
        Exti6wflW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wfl(&mut self) -> Exti7wflW<WakupflgSpec> {
        Exti7wflW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wfl(&mut self) -> Exti8wflW<WakupflgSpec> {
        Exti8wflW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wfl(&mut self) -> Exti9wflW<WakupflgSpec> {
        Exti9wflW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wfl(&mut self) -> Exti10wflW<WakupflgSpec> {
        Exti10wflW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wfl(&mut self) -> Exti11wflW<WakupflgSpec> {
        Exti11wflW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wfl(&mut self) -> Exti12wflW<WakupflgSpec> {
        Exti12wflW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wfl(&mut self) -> Exti13wflW<WakupflgSpec> {
        Exti13wflW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wfl(&mut self) -> Exti14wflW<WakupflgSpec> {
        Exti14wflW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wfl(&mut self) -> Exti15wflW<WakupflgSpec> {
        Exti15wflW::new(self, 15)
    }
}
#[doc = "WAKUPFLG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakupflg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakupflg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakupflgSpec;
impl crate::RegisterSpec for WakupflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakupflg::R`](R) reader structure"]
impl crate::Readable for WakupflgSpec {}
#[doc = "`write(|w| ..)` method takes [`wakupflg::W`](W) writer structure"]
impl crate::Writable for WakupflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKUPFLG to value 0"]
impl crate::Resettable for WakupflgSpec {
    const RESET_VALUE: u32 = 0;
}
