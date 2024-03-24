#[doc = "Register `SSCR` reader"]
pub type R = crate::R<SscrSpec>;
#[doc = "Register `SSCR` writer"]
pub type W = crate::W<SscrSpec>;
#[doc = "Field `EXTI0SC` reader - EXTI0SC"]
pub type Exti0scR = crate::BitReader;
#[doc = "Field `EXTI0SC` writer - EXTI0SC"]
pub type Exti0scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1SC` reader - EXTI1SC"]
pub type Exti1scR = crate::BitReader;
#[doc = "Field `EXTI1SC` writer - EXTI1SC"]
pub type Exti1scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2SC` reader - EXTI2SC"]
pub type Exti2scR = crate::BitReader;
#[doc = "Field `EXTI2SC` writer - EXTI2SC"]
pub type Exti2scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3SC` reader - EXTI3SC"]
pub type Exti3scR = crate::BitReader;
#[doc = "Field `EXTI3SC` writer - EXTI3SC"]
pub type Exti3scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4SC` reader - EXTI4SC"]
pub type Exti4scR = crate::BitReader;
#[doc = "Field `EXTI4SC` writer - EXTI4SC"]
pub type Exti4scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5SC` reader - EXTI5SC"]
pub type Exti5scR = crate::BitReader;
#[doc = "Field `EXTI5SC` writer - EXTI5SC"]
pub type Exti5scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6SC` reader - EXTI6SC"]
pub type Exti6scR = crate::BitReader;
#[doc = "Field `EXTI6SC` writer - EXTI6SC"]
pub type Exti6scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7SC` reader - EXTI7SC"]
pub type Exti7scR = crate::BitReader;
#[doc = "Field `EXTI7SC` writer - EXTI7SC"]
pub type Exti7scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8SC` reader - EXTI8SC"]
pub type Exti8scR = crate::BitReader;
#[doc = "Field `EXTI8SC` writer - EXTI8SC"]
pub type Exti8scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9SC` reader - EXTI9SC"]
pub type Exti9scR = crate::BitReader;
#[doc = "Field `EXTI9SC` writer - EXTI9SC"]
pub type Exti9scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10SC` reader - EXTI10SC"]
pub type Exti10scR = crate::BitReader;
#[doc = "Field `EXTI10SC` writer - EXTI10SC"]
pub type Exti10scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11SC` reader - EXTI11SC"]
pub type Exti11scR = crate::BitReader;
#[doc = "Field `EXTI11SC` writer - EXTI11SC"]
pub type Exti11scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12SC` reader - EXTI12SC"]
pub type Exti12scR = crate::BitReader;
#[doc = "Field `EXTI12SC` writer - EXTI12SC"]
pub type Exti12scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13SC` reader - EXTI13SC"]
pub type Exti13scR = crate::BitReader;
#[doc = "Field `EXTI13SC` writer - EXTI13SC"]
pub type Exti13scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14SC` reader - EXTI14SC"]
pub type Exti14scR = crate::BitReader;
#[doc = "Field `EXTI14SC` writer - EXTI14SC"]
pub type Exti14scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15SC` reader - EXTI15SC"]
pub type Exti15scR = crate::BitReader;
#[doc = "Field `EXTI15SC` writer - EXTI15SC"]
pub type Exti15scW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    pub fn exti0sc(&self) -> Exti0scR {
        Exti0scR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    pub fn exti1sc(&self) -> Exti1scR {
        Exti1scR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    pub fn exti2sc(&self) -> Exti2scR {
        Exti2scR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    pub fn exti3sc(&self) -> Exti3scR {
        Exti3scR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    pub fn exti4sc(&self) -> Exti4scR {
        Exti4scR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    pub fn exti5sc(&self) -> Exti5scR {
        Exti5scR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    pub fn exti6sc(&self) -> Exti6scR {
        Exti6scR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    pub fn exti7sc(&self) -> Exti7scR {
        Exti7scR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    pub fn exti8sc(&self) -> Exti8scR {
        Exti8scR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    pub fn exti9sc(&self) -> Exti9scR {
        Exti9scR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    pub fn exti10sc(&self) -> Exti10scR {
        Exti10scR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    pub fn exti11sc(&self) -> Exti11scR {
        Exti11scR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    pub fn exti12sc(&self) -> Exti12scR {
        Exti12scR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    pub fn exti13sc(&self) -> Exti13scR {
        Exti13scR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    pub fn exti14sc(&self) -> Exti14scR {
        Exti14scR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    pub fn exti15sc(&self) -> Exti15scR {
        Exti15scR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti0sc(&mut self) -> Exti0scW<SscrSpec> {
        Exti0scW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti1sc(&mut self) -> Exti1scW<SscrSpec> {
        Exti1scW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti2sc(&mut self) -> Exti2scW<SscrSpec> {
        Exti2scW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti3sc(&mut self) -> Exti3scW<SscrSpec> {
        Exti3scW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti4sc(&mut self) -> Exti4scW<SscrSpec> {
        Exti4scW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti5sc(&mut self) -> Exti5scW<SscrSpec> {
        Exti5scW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti6sc(&mut self) -> Exti6scW<SscrSpec> {
        Exti6scW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti7sc(&mut self) -> Exti7scW<SscrSpec> {
        Exti7scW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti8sc(&mut self) -> Exti8scW<SscrSpec> {
        Exti8scW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti9sc(&mut self) -> Exti9scW<SscrSpec> {
        Exti9scW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti10sc(&mut self) -> Exti10scW<SscrSpec> {
        Exti10scW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti11sc(&mut self) -> Exti11scW<SscrSpec> {
        Exti11scW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti12sc(&mut self) -> Exti12scW<SscrSpec> {
        Exti12scW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti13sc(&mut self) -> Exti13scW<SscrSpec> {
        Exti13scW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti14sc(&mut self) -> Exti14scW<SscrSpec> {
        Exti14scW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    #[must_use]
    pub fn exti15sc(&mut self) -> Exti15scW<SscrSpec> {
        Exti15scW::new(self, 15)
    }
}
#[doc = "SSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SscrSpec;
impl crate::RegisterSpec for SscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sscr::R`](R) reader structure"]
impl crate::Readable for SscrSpec {}
#[doc = "`write(|w| ..)` method takes [`sscr::W`](W) writer structure"]
impl crate::Writable for SscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSCR to value 0"]
impl crate::Resettable for SscrSpec {
    const RESET_VALUE: u32 = 0;
}
