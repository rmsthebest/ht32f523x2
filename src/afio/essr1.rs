#[doc = "Register `ESSR1` reader"]
pub type R = crate::R<Essr1Spec>;
#[doc = "Register `ESSR1` writer"]
pub type W = crate::W<Essr1Spec>;
#[doc = "Field `EXTI8PIN` reader - EXTI8PIN"]
pub type Exti8pinR = crate::FieldReader;
#[doc = "Field `EXTI8PIN` writer - EXTI8PIN"]
pub type Exti8pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI9PIN` reader - EXTI9PIN"]
pub type Exti9pinR = crate::FieldReader;
#[doc = "Field `EXTI9PIN` writer - EXTI9PIN"]
pub type Exti9pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI10PIN` reader - EXTI10PIN"]
pub type Exti10pinR = crate::FieldReader;
#[doc = "Field `EXTI10PIN` writer - EXTI10PIN"]
pub type Exti10pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI11PIN` reader - EXTI11PIN"]
pub type Exti11pinR = crate::FieldReader;
#[doc = "Field `EXTI11PIN` writer - EXTI11PIN"]
pub type Exti11pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI12PIN` reader - EXTI12PIN"]
pub type Exti12pinR = crate::FieldReader;
#[doc = "Field `EXTI12PIN` writer - EXTI12PIN"]
pub type Exti12pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI13PIN` reader - EXTI13PIN"]
pub type Exti13pinR = crate::FieldReader;
#[doc = "Field `EXTI13PIN` writer - EXTI13PIN"]
pub type Exti13pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI14PIN` reader - EXTI14PIN"]
pub type Exti14pinR = crate::FieldReader;
#[doc = "Field `EXTI14PIN` writer - EXTI14PIN"]
pub type Exti14pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI15PIN` reader - EXTI15PIN"]
pub type Exti15pinR = crate::FieldReader;
#[doc = "Field `EXTI15PIN` writer - EXTI15PIN"]
pub type Exti15pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    pub fn exti8pin(&self) -> Exti8pinR {
        Exti8pinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    pub fn exti9pin(&self) -> Exti9pinR {
        Exti9pinR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    pub fn exti10pin(&self) -> Exti10pinR {
        Exti10pinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    pub fn exti11pin(&self) -> Exti11pinR {
        Exti11pinR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    pub fn exti12pin(&self) -> Exti12pinR {
        Exti12pinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    pub fn exti13pin(&self) -> Exti13pinR {
        Exti13pinR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    pub fn exti14pin(&self) -> Exti14pinR {
        Exti14pinR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    pub fn exti15pin(&self) -> Exti15pinR {
        Exti15pinR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti8pin(&mut self) -> Exti8pinW<Essr1Spec> {
        Exti8pinW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti9pin(&mut self) -> Exti9pinW<Essr1Spec> {
        Exti9pinW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti10pin(&mut self) -> Exti10pinW<Essr1Spec> {
        Exti10pinW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti11pin(&mut self) -> Exti11pinW<Essr1Spec> {
        Exti11pinW::new(self, 12)
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti12pin(&mut self) -> Exti12pinW<Essr1Spec> {
        Exti12pinW::new(self, 16)
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti13pin(&mut self) -> Exti13pinW<Essr1Spec> {
        Exti13pinW::new(self, 20)
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti14pin(&mut self) -> Exti14pinW<Essr1Spec> {
        Exti14pinW::new(self, 24)
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti15pin(&mut self) -> Exti15pinW<Essr1Spec> {
        Exti15pinW::new(self, 28)
    }
}
#[doc = "ESSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`essr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`essr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Essr1Spec;
impl crate::RegisterSpec for Essr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`essr1::R`](R) reader structure"]
impl crate::Readable for Essr1Spec {}
#[doc = "`write(|w| ..)` method takes [`essr1::W`](W) writer structure"]
impl crate::Writable for Essr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESSR1 to value 0"]
impl crate::Resettable for Essr1Spec {
    const RESET_VALUE: u32 = 0;
}
