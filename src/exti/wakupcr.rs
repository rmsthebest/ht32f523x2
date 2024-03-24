#[doc = "Register `WAKUPCR` reader"]
pub type R = crate::R<WakupcrSpec>;
#[doc = "Register `WAKUPCR` writer"]
pub type W = crate::W<WakupcrSpec>;
#[doc = "Field `EXTI0WEN` reader - EXTI0WEN"]
pub type Exti0wenR = crate::BitReader;
#[doc = "Field `EXTI0WEN` writer - EXTI0WEN"]
pub type Exti0wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1WEN` reader - EXTI1WEN"]
pub type Exti1wenR = crate::BitReader;
#[doc = "Field `EXTI1WEN` writer - EXTI1WEN"]
pub type Exti1wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2WEN` reader - EXTI2WEN"]
pub type Exti2wenR = crate::BitReader;
#[doc = "Field `EXTI2WEN` writer - EXTI2WEN"]
pub type Exti2wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3WEN` reader - EXTI3WEN"]
pub type Exti3wenR = crate::BitReader;
#[doc = "Field `EXTI3WEN` writer - EXTI3WEN"]
pub type Exti3wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4WEN` reader - EXTI4WEN"]
pub type Exti4wenR = crate::BitReader;
#[doc = "Field `EXTI4WEN` writer - EXTI4WEN"]
pub type Exti4wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5WEN` reader - EXTI5WEN"]
pub type Exti5wenR = crate::BitReader;
#[doc = "Field `EXTI5WEN` writer - EXTI5WEN"]
pub type Exti5wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6WEN` reader - EXTI6WEN"]
pub type Exti6wenR = crate::BitReader;
#[doc = "Field `EXTI6WEN` writer - EXTI6WEN"]
pub type Exti6wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7WEN` reader - EXTI7WEN"]
pub type Exti7wenR = crate::BitReader;
#[doc = "Field `EXTI7WEN` writer - EXTI7WEN"]
pub type Exti7wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8WEN` reader - EXTI8WEN"]
pub type Exti8wenR = crate::BitReader;
#[doc = "Field `EXTI8WEN` writer - EXTI8WEN"]
pub type Exti8wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9WEN` reader - EXTI9WEN"]
pub type Exti9wenR = crate::BitReader;
#[doc = "Field `EXTI9WEN` writer - EXTI9WEN"]
pub type Exti9wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10WEN` reader - EXTI10WEN"]
pub type Exti10wenR = crate::BitReader;
#[doc = "Field `EXTI10WEN` writer - EXTI10WEN"]
pub type Exti10wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11WEN` reader - EXTI11WEN"]
pub type Exti11wenR = crate::BitReader;
#[doc = "Field `EXTI11WEN` writer - EXTI11WEN"]
pub type Exti11wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12WEN` reader - EXTI12WEN"]
pub type Exti12wenR = crate::BitReader;
#[doc = "Field `EXTI12WEN` writer - EXTI12WEN"]
pub type Exti12wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13WEN` reader - EXTI13WEN"]
pub type Exti13wenR = crate::BitReader;
#[doc = "Field `EXTI13WEN` writer - EXTI13WEN"]
pub type Exti13wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14WEN` reader - EXTI14WEN"]
pub type Exti14wenR = crate::BitReader;
#[doc = "Field `EXTI14WEN` writer - EXTI14WEN"]
pub type Exti14wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15WEN` reader - EXTI15WEN"]
pub type Exti15wenR = crate::BitReader;
#[doc = "Field `EXTI15WEN` writer - EXTI15WEN"]
pub type Exti15wenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVWUPIEN` reader - EVWUPIEN"]
pub type EvwupienR = crate::BitReader;
#[doc = "Field `EVWUPIEN` writer - EVWUPIEN"]
pub type EvwupienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    pub fn exti0wen(&self) -> Exti0wenR {
        Exti0wenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    pub fn exti1wen(&self) -> Exti1wenR {
        Exti1wenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    pub fn exti2wen(&self) -> Exti2wenR {
        Exti2wenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    pub fn exti3wen(&self) -> Exti3wenR {
        Exti3wenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    pub fn exti4wen(&self) -> Exti4wenR {
        Exti4wenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    pub fn exti5wen(&self) -> Exti5wenR {
        Exti5wenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    pub fn exti6wen(&self) -> Exti6wenR {
        Exti6wenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    pub fn exti7wen(&self) -> Exti7wenR {
        Exti7wenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    pub fn exti8wen(&self) -> Exti8wenR {
        Exti8wenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    pub fn exti9wen(&self) -> Exti9wenR {
        Exti9wenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    pub fn exti10wen(&self) -> Exti10wenR {
        Exti10wenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    pub fn exti11wen(&self) -> Exti11wenR {
        Exti11wenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    pub fn exti12wen(&self) -> Exti12wenR {
        Exti12wenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    pub fn exti13wen(&self) -> Exti13wenR {
        Exti13wenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    pub fn exti14wen(&self) -> Exti14wenR {
        Exti14wenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    pub fn exti15wen(&self) -> Exti15wenR {
        Exti15wenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    pub fn evwupien(&self) -> EvwupienR {
        EvwupienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0wen(&mut self) -> Exti0wenW<WakupcrSpec> {
        Exti0wenW::new(self, 0)
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1wen(&mut self) -> Exti1wenW<WakupcrSpec> {
        Exti1wenW::new(self, 1)
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2wen(&mut self) -> Exti2wenW<WakupcrSpec> {
        Exti2wenW::new(self, 2)
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3wen(&mut self) -> Exti3wenW<WakupcrSpec> {
        Exti3wenW::new(self, 3)
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4wen(&mut self) -> Exti4wenW<WakupcrSpec> {
        Exti4wenW::new(self, 4)
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5wen(&mut self) -> Exti5wenW<WakupcrSpec> {
        Exti5wenW::new(self, 5)
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6wen(&mut self) -> Exti6wenW<WakupcrSpec> {
        Exti6wenW::new(self, 6)
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7wen(&mut self) -> Exti7wenW<WakupcrSpec> {
        Exti7wenW::new(self, 7)
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8wen(&mut self) -> Exti8wenW<WakupcrSpec> {
        Exti8wenW::new(self, 8)
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9wen(&mut self) -> Exti9wenW<WakupcrSpec> {
        Exti9wenW::new(self, 9)
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10wen(&mut self) -> Exti10wenW<WakupcrSpec> {
        Exti10wenW::new(self, 10)
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11wen(&mut self) -> Exti11wenW<WakupcrSpec> {
        Exti11wenW::new(self, 11)
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12wen(&mut self) -> Exti12wenW<WakupcrSpec> {
        Exti12wenW::new(self, 12)
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13wen(&mut self) -> Exti13wenW<WakupcrSpec> {
        Exti13wenW::new(self, 13)
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14wen(&mut self) -> Exti14wenW<WakupcrSpec> {
        Exti14wenW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15wen(&mut self) -> Exti15wenW<WakupcrSpec> {
        Exti15wenW::new(self, 15)
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn evwupien(&mut self) -> EvwupienW<WakupcrSpec> {
        EvwupienW::new(self, 31)
    }
}
#[doc = "WAKUPCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakupcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakupcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakupcrSpec;
impl crate::RegisterSpec for WakupcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakupcr::R`](R) reader structure"]
impl crate::Readable for WakupcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wakupcr::W`](W) writer structure"]
impl crate::Writable for WakupcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKUPCR to value 0"]
impl crate::Resettable for WakupcrSpec {
    const RESET_VALUE: u32 = 0;
}
