#[doc = "Register `DR5` reader"]
pub type R = crate::R<Dr5Spec>;
#[doc = "Register `DR5` writer"]
pub type W = crate::W<Dr5Spec>;
#[doc = "Field `ADD5` reader - ADD5"]
pub type Add5R = crate::FieldReader<u16>;
#[doc = "Field `ADD5` writer - ADD5"]
pub type Add5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD5` reader - ADVLD5"]
pub type Advld5R = crate::BitReader;
#[doc = "Field `ADVLD5` writer - ADVLD5"]
pub type Advld5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    pub fn add5(&self) -> Add5R {
        Add5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    pub fn advld5(&self) -> Advld5R {
        Advld5R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    #[must_use]
    pub fn add5(&mut self) -> Add5W<Dr5Spec> {
        Add5W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    #[must_use]
    pub fn advld5(&mut self) -> Advld5W<Dr5Spec> {
        Advld5W::new(self, 31)
    }
}
#[doc = "DR5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr5Spec;
impl crate::RegisterSpec for Dr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr5::R`](R) reader structure"]
impl crate::Readable for Dr5Spec {}
#[doc = "`write(|w| ..)` method takes [`dr5::W`](W) writer structure"]
impl crate::Writable for Dr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR5 to value 0"]
impl crate::Resettable for Dr5Spec {
    const RESET_VALUE: u32 = 0;
}
