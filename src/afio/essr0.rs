#[doc = "Register `ESSR0` reader"]
pub type R = crate::R<Essr0Spec>;
#[doc = "Register `ESSR0` writer"]
pub type W = crate::W<Essr0Spec>;
#[doc = "Field `EXTI0PIN` reader - EXTI0PIN"]
pub type Exti0pinR = crate::FieldReader;
#[doc = "Field `EXTI0PIN` writer - EXTI0PIN"]
pub type Exti0pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI1PIN` reader - EXTI1PIN"]
pub type Exti1pinR = crate::FieldReader;
#[doc = "Field `EXTI1PIN` writer - EXTI1PIN"]
pub type Exti1pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI2PIN` reader - EXTI2PIN"]
pub type Exti2pinR = crate::FieldReader;
#[doc = "Field `EXTI2PIN` writer - EXTI2PIN"]
pub type Exti2pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI3PIN` reader - EXTI3PIN"]
pub type Exti3pinR = crate::FieldReader;
#[doc = "Field `EXTI3PIN` writer - EXTI3PIN"]
pub type Exti3pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI4PIN` reader - EXTI4PIN"]
pub type Exti4pinR = crate::FieldReader;
#[doc = "Field `EXTI4PIN` writer - EXTI4PIN"]
pub type Exti4pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI5PIN` reader - EXTI5PIN"]
pub type Exti5pinR = crate::FieldReader;
#[doc = "Field `EXTI5PIN` writer - EXTI5PIN"]
pub type Exti5pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI6PIN` reader - EXTI6PIN"]
pub type Exti6pinR = crate::FieldReader;
#[doc = "Field `EXTI6PIN` writer - EXTI6PIN"]
pub type Exti6pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI7PIN` reader - EXTI7PIN"]
pub type Exti7pinR = crate::FieldReader;
#[doc = "Field `EXTI7PIN` writer - EXTI7PIN"]
pub type Exti7pinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    pub fn exti0pin(&self) -> Exti0pinR {
        Exti0pinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    pub fn exti1pin(&self) -> Exti1pinR {
        Exti1pinR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    pub fn exti2pin(&self) -> Exti2pinR {
        Exti2pinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    pub fn exti3pin(&self) -> Exti3pinR {
        Exti3pinR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    pub fn exti4pin(&self) -> Exti4pinR {
        Exti4pinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    pub fn exti5pin(&self) -> Exti5pinR {
        Exti5pinR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    pub fn exti6pin(&self) -> Exti6pinR {
        Exti6pinR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    pub fn exti7pin(&self) -> Exti7pinR {
        Exti7pinR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti0pin(&mut self) -> Exti0pinW<Essr0Spec> {
        Exti0pinW::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti1pin(&mut self) -> Exti1pinW<Essr0Spec> {
        Exti1pinW::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti2pin(&mut self) -> Exti2pinW<Essr0Spec> {
        Exti2pinW::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti3pin(&mut self) -> Exti3pinW<Essr0Spec> {
        Exti3pinW::new(self, 12)
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti4pin(&mut self) -> Exti4pinW<Essr0Spec> {
        Exti4pinW::new(self, 16)
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti5pin(&mut self) -> Exti5pinW<Essr0Spec> {
        Exti5pinW::new(self, 20)
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti6pin(&mut self) -> Exti6pinW<Essr0Spec> {
        Exti6pinW::new(self, 24)
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    #[must_use]
    pub fn exti7pin(&mut self) -> Exti7pinW<Essr0Spec> {
        Exti7pinW::new(self, 28)
    }
}
#[doc = "ESSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`essr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`essr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Essr0Spec;
impl crate::RegisterSpec for Essr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`essr0::R`](R) reader structure"]
impl crate::Readable for Essr0Spec {}
#[doc = "`write(|w| ..)` method takes [`essr0::W`](W) writer structure"]
impl crate::Writable for Essr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESSR0 to value 0"]
impl crate::Resettable for Essr0Spec {
    const RESET_VALUE: u32 = 0;
}
