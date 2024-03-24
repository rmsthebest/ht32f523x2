#[doc = "Register `EDGESR` reader"]
pub type R = crate::R<EdgesrSpec>;
#[doc = "Register `EDGESR` writer"]
pub type W = crate::W<EdgesrSpec>;
#[doc = "Field `EXTI0EDS` reader - EXTI0EDS"]
pub type Exti0edsR = crate::BitReader;
#[doc = "Field `EXTI0EDS` writer - EXTI0EDS"]
pub type Exti0edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1EDS` reader - EXTI1EDS"]
pub type Exti1edsR = crate::BitReader;
#[doc = "Field `EXTI1EDS` writer - EXTI1EDS"]
pub type Exti1edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2EDS` reader - EXTI2EDS"]
pub type Exti2edsR = crate::BitReader;
#[doc = "Field `EXTI2EDS` writer - EXTI2EDS"]
pub type Exti2edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3EDS` reader - EXTI3EDS"]
pub type Exti3edsR = crate::BitReader;
#[doc = "Field `EXTI3EDS` writer - EXTI3EDS"]
pub type Exti3edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4EDS` reader - EXTI4EDS"]
pub type Exti4edsR = crate::BitReader;
#[doc = "Field `EXTI4EDS` writer - EXTI4EDS"]
pub type Exti4edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5EDS` reader - EXTI5EDS"]
pub type Exti5edsR = crate::BitReader;
#[doc = "Field `EXTI5EDS` writer - EXTI5EDS"]
pub type Exti5edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6EDS` reader - EXTI6EDS"]
pub type Exti6edsR = crate::BitReader;
#[doc = "Field `EXTI6EDS` writer - EXTI6EDS"]
pub type Exti6edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7EDS` reader - EXTI7EDS"]
pub type Exti7edsR = crate::BitReader;
#[doc = "Field `EXTI7EDS` writer - EXTI7EDS"]
pub type Exti7edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8EDS` reader - EXTI8EDS"]
pub type Exti8edsR = crate::BitReader;
#[doc = "Field `EXTI8EDS` writer - EXTI8EDS"]
pub type Exti8edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9EDS` reader - EXTI9EDS"]
pub type Exti9edsR = crate::BitReader;
#[doc = "Field `EXTI9EDS` writer - EXTI9EDS"]
pub type Exti9edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10EDS` reader - EXTI10EDS"]
pub type Exti10edsR = crate::BitReader;
#[doc = "Field `EXTI10EDS` writer - EXTI10EDS"]
pub type Exti10edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11EDS` reader - EXTI11EDS"]
pub type Exti11edsR = crate::BitReader;
#[doc = "Field `EXTI11EDS` writer - EXTI11EDS"]
pub type Exti11edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12EDS` reader - EXTI12EDS"]
pub type Exti12edsR = crate::BitReader;
#[doc = "Field `EXTI12EDS` writer - EXTI12EDS"]
pub type Exti12edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13EDS` reader - EXTI13EDS"]
pub type Exti13edsR = crate::BitReader;
#[doc = "Field `EXTI13EDS` writer - EXTI13EDS"]
pub type Exti13edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14EDS` reader - EXTI14EDS"]
pub type Exti14edsR = crate::BitReader;
#[doc = "Field `EXTI14EDS` writer - EXTI14EDS"]
pub type Exti14edsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15EDS` reader - EXTI15EDS"]
pub type Exti15edsR = crate::BitReader;
#[doc = "Field `EXTI15EDS` writer - EXTI15EDS"]
pub type Exti15edsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    pub fn exti0eds(&self) -> Exti0edsR {
        Exti0edsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    pub fn exti1eds(&self) -> Exti1edsR {
        Exti1edsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    pub fn exti2eds(&self) -> Exti2edsR {
        Exti2edsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    pub fn exti3eds(&self) -> Exti3edsR {
        Exti3edsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    pub fn exti4eds(&self) -> Exti4edsR {
        Exti4edsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    pub fn exti5eds(&self) -> Exti5edsR {
        Exti5edsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    pub fn exti6eds(&self) -> Exti6edsR {
        Exti6edsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    pub fn exti7eds(&self) -> Exti7edsR {
        Exti7edsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    pub fn exti8eds(&self) -> Exti8edsR {
        Exti8edsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    pub fn exti9eds(&self) -> Exti9edsR {
        Exti9edsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    pub fn exti10eds(&self) -> Exti10edsR {
        Exti10edsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    pub fn exti11eds(&self) -> Exti11edsR {
        Exti11edsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    pub fn exti12eds(&self) -> Exti12edsR {
        Exti12edsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    pub fn exti13eds(&self) -> Exti13edsR {
        Exti13edsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    pub fn exti14eds(&self) -> Exti14edsR {
        Exti14edsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    pub fn exti15eds(&self) -> Exti15edsR {
        Exti15edsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti0eds(&mut self) -> Exti0edsW<EdgesrSpec> {
        Exti0edsW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti1eds(&mut self) -> Exti1edsW<EdgesrSpec> {
        Exti1edsW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti2eds(&mut self) -> Exti2edsW<EdgesrSpec> {
        Exti2edsW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti3eds(&mut self) -> Exti3edsW<EdgesrSpec> {
        Exti3edsW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti4eds(&mut self) -> Exti4edsW<EdgesrSpec> {
        Exti4edsW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti5eds(&mut self) -> Exti5edsW<EdgesrSpec> {
        Exti5edsW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti6eds(&mut self) -> Exti6edsW<EdgesrSpec> {
        Exti6edsW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti7eds(&mut self) -> Exti7edsW<EdgesrSpec> {
        Exti7edsW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti8eds(&mut self) -> Exti8edsW<EdgesrSpec> {
        Exti8edsW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti9eds(&mut self) -> Exti9edsW<EdgesrSpec> {
        Exti9edsW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti10eds(&mut self) -> Exti10edsW<EdgesrSpec> {
        Exti10edsW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti11eds(&mut self) -> Exti11edsW<EdgesrSpec> {
        Exti11edsW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti12eds(&mut self) -> Exti12edsW<EdgesrSpec> {
        Exti12edsW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti13eds(&mut self) -> Exti13edsW<EdgesrSpec> {
        Exti13edsW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti14eds(&mut self) -> Exti14edsW<EdgesrSpec> {
        Exti14edsW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    #[must_use]
    pub fn exti15eds(&mut self) -> Exti15edsW<EdgesrSpec> {
        Exti15edsW::new(self, 15)
    }
}
#[doc = "EDGESR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edgesr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edgesr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdgesrSpec;
impl crate::RegisterSpec for EdgesrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edgesr::R`](R) reader structure"]
impl crate::Readable for EdgesrSpec {}
#[doc = "`write(|w| ..)` method takes [`edgesr::W`](W) writer structure"]
impl crate::Writable for EdgesrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDGESR to value 0"]
impl crate::Resettable for EdgesrSpec {
    const RESET_VALUE: u32 = 0;
}
