#[doc = "Register `WAKUPPOLR` reader"]
pub type R = crate::R<WakuppolrSpec>;
#[doc = "Register `WAKUPPOLR` writer"]
pub type W = crate::W<WakuppolrSpec>;
#[doc = "Field `EXTI0WPOL` reader - EXTI0WPOL"]
pub type Exti0wpolR = crate::BitReader;
#[doc = "Field `EXTI0WPOL` writer - EXTI0WPOL"]
pub type Exti0wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1WPOL` reader - EXTI1WPOL"]
pub type Exti1wpolR = crate::BitReader;
#[doc = "Field `EXTI1WPOL` writer - EXTI1WPOL"]
pub type Exti1wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2WPOL` reader - EXTI2WPOL"]
pub type Exti2wpolR = crate::BitReader;
#[doc = "Field `EXTI2WPOL` writer - EXTI2WPOL"]
pub type Exti2wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3WPOL` reader - EXTI3WPOL"]
pub type Exti3wpolR = crate::BitReader;
#[doc = "Field `EXTI3WPOL` writer - EXTI3WPOL"]
pub type Exti3wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4WPOL` reader - EXTI4WPOL"]
pub type Exti4wpolR = crate::BitReader;
#[doc = "Field `EXTI4WPOL` writer - EXTI4WPOL"]
pub type Exti4wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5WPOL` reader - EXTI5WPOL"]
pub type Exti5wpolR = crate::BitReader;
#[doc = "Field `EXTI5WPOL` writer - EXTI5WPOL"]
pub type Exti5wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6WPOL` reader - EXTI6WPOL"]
pub type Exti6wpolR = crate::BitReader;
#[doc = "Field `EXTI6WPOL` writer - EXTI6WPOL"]
pub type Exti6wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7WPOL` reader - EXTI7WPOL"]
pub type Exti7wpolR = crate::BitReader;
#[doc = "Field `EXTI7WPOL` writer - EXTI7WPOL"]
pub type Exti7wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8WPOL` reader - EXTI8WPOL"]
pub type Exti8wpolR = crate::BitReader;
#[doc = "Field `EXTI8WPOL` writer - EXTI8WPOL"]
pub type Exti8wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9WPOL` reader - EXTI9WPOL"]
pub type Exti9wpolR = crate::BitReader;
#[doc = "Field `EXTI9WPOL` writer - EXTI9WPOL"]
pub type Exti9wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10WPOL` reader - EXTI10WPOL"]
pub type Exti10wpolR = crate::BitReader;
#[doc = "Field `EXTI10WPOL` writer - EXTI10WPOL"]
pub type Exti10wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11WPOL` reader - EXTI11WPOL"]
pub type Exti11wpolR = crate::BitReader;
#[doc = "Field `EXTI11WPOL` writer - EXTI11WPOL"]
pub type Exti11wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12WPOL` reader - EXTI12WPOL"]
pub type Exti12wpolR = crate::BitReader;
#[doc = "Field `EXTI12WPOL` writer - EXTI12WPOL"]
pub type Exti12wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13WPOL` reader - EXTI13WPOL"]
pub type Exti13wpolR = crate::BitReader;
#[doc = "Field `EXTI13WPOL` writer - EXTI13WPOL"]
pub type Exti13wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14WPOL` reader - EXTI14WPOL"]
pub type Exti14wpolR = crate::BitReader;
#[doc = "Field `EXTI14WPOL` writer - EXTI14WPOL"]
pub type Exti14wpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15WPOL` reader - EXTI15WPOL"]
pub type Exti15wpolR = crate::BitReader;
#[doc = "Field `EXTI15WPOL` writer - EXTI15WPOL"]
pub type Exti15wpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    pub fn exti0wpol(&self) -> Exti0wpolR {
        Exti0wpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    pub fn exti1wpol(&self) -> Exti1wpolR {
        Exti1wpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    pub fn exti2wpol(&self) -> Exti2wpolR {
        Exti2wpolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    pub fn exti3wpol(&self) -> Exti3wpolR {
        Exti3wpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    pub fn exti4wpol(&self) -> Exti4wpolR {
        Exti4wpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    pub fn exti5wpol(&self) -> Exti5wpolR {
        Exti5wpolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    pub fn exti6wpol(&self) -> Exti6wpolR {
        Exti6wpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    pub fn exti7wpol(&self) -> Exti7wpolR {
        Exti7wpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    pub fn exti8wpol(&self) -> Exti8wpolR {
        Exti8wpolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    pub fn exti9wpol(&self) -> Exti9wpolR {
        Exti9wpolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    pub fn exti10wpol(&self) -> Exti10wpolR {
        Exti10wpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    pub fn exti11wpol(&self) -> Exti11wpolR {
        Exti11wpolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    pub fn exti12wpol(&self) -> Exti12wpolR {
        Exti12wpolR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    pub fn exti13wpol(&self) -> Exti13wpolR {
        Exti13wpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    pub fn exti14wpol(&self) -> Exti14wpolR {
        Exti14wpolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    pub fn exti15wpol(&self) -> Exti15wpolR {
        Exti15wpolR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wpol(&mut self) -> Exti0wpolW<WakuppolrSpec> {
        Exti0wpolW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wpol(&mut self) -> Exti1wpolW<WakuppolrSpec> {
        Exti1wpolW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wpol(&mut self) -> Exti2wpolW<WakuppolrSpec> {
        Exti2wpolW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wpol(&mut self) -> Exti3wpolW<WakuppolrSpec> {
        Exti3wpolW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wpol(&mut self) -> Exti4wpolW<WakuppolrSpec> {
        Exti4wpolW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wpol(&mut self) -> Exti5wpolW<WakuppolrSpec> {
        Exti5wpolW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wpol(&mut self) -> Exti6wpolW<WakuppolrSpec> {
        Exti6wpolW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wpol(&mut self) -> Exti7wpolW<WakuppolrSpec> {
        Exti7wpolW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wpol(&mut self) -> Exti8wpolW<WakuppolrSpec> {
        Exti8wpolW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wpol(&mut self) -> Exti9wpolW<WakuppolrSpec> {
        Exti9wpolW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wpol(&mut self) -> Exti10wpolW<WakuppolrSpec> {
        Exti10wpolW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wpol(&mut self) -> Exti11wpolW<WakuppolrSpec> {
        Exti11wpolW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wpol(&mut self) -> Exti12wpolW<WakuppolrSpec> {
        Exti12wpolW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wpol(&mut self) -> Exti13wpolW<WakuppolrSpec> {
        Exti13wpolW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wpol(&mut self) -> Exti14wpolW<WakuppolrSpec> {
        Exti14wpolW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wpol(&mut self) -> Exti15wpolW<WakuppolrSpec> {
        Exti15wpolW::new(self, 15)
    }
}
#[doc = "WAKUPPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakuppolr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakuppolr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakuppolrSpec;
impl crate::RegisterSpec for WakuppolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakuppolr::R`](R) reader structure"]
impl crate::Readable for WakuppolrSpec {}
#[doc = "`write(|w| ..)` method takes [`wakuppolr::W`](W) writer structure"]
impl crate::Writable for WakuppolrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKUPPOLR to value 0"]
impl crate::Resettable for WakuppolrSpec {
    const RESET_VALUE: u32 = 0;
}
