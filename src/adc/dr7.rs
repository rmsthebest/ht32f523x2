#[doc = "Register `DR7` reader"]
pub type R = crate::R<Dr7Spec>;
#[doc = "Register `DR7` writer"]
pub type W = crate::W<Dr7Spec>;
#[doc = "Field `ADD7` reader - ADD7"]
pub type Add7R = crate::FieldReader<u16>;
#[doc = "Field `ADD7` writer - ADD7"]
pub type Add7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD7` reader - ADVLD7"]
pub type Advld7R = crate::BitReader;
#[doc = "Field `ADVLD7` writer - ADVLD7"]
pub type Advld7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD7"]
    #[inline(always)]
    pub fn add7(&self) -> Add7R {
        Add7R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD7"]
    #[inline(always)]
    pub fn advld7(&self) -> Advld7R {
        Advld7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD7"]
    #[inline(always)]
    pub fn add7(&mut self) -> Add7W<'_, Dr7Spec> {
        Add7W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD7"]
    #[inline(always)]
    pub fn advld7(&mut self) -> Advld7W<'_, Dr7Spec> {
        Advld7W::new(self, 31)
    }
}
#[doc = "DR7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr7Spec;
impl crate::RegisterSpec for Dr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr7::R`](R) reader structure"]
impl crate::Readable for Dr7Spec {}
#[doc = "`write(|w| ..)` method takes [`dr7::W`](W) writer structure"]
impl crate::Writable for Dr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR7 to value 0"]
impl crate::Resettable for Dr7Spec {}
