#[doc = "Register `DR1` reader"]
pub type R = crate::R<Dr1Spec>;
#[doc = "Register `DR1` writer"]
pub type W = crate::W<Dr1Spec>;
#[doc = "Field `ADD1` reader - ADD1"]
pub type Add1R = crate::FieldReader<u16>;
#[doc = "Field `ADD1` writer - ADD1"]
pub type Add1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD1` reader - ADVLD1"]
pub type Advld1R = crate::BitReader;
#[doc = "Field `ADVLD1` writer - ADVLD1"]
pub type Advld1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    pub fn add1(&self) -> Add1R {
        Add1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    pub fn advld1(&self) -> Advld1R {
        Advld1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    #[must_use]
    pub fn add1(&mut self) -> Add1W<Dr1Spec> {
        Add1W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    #[must_use]
    pub fn advld1(&mut self) -> Advld1W<Dr1Spec> {
        Advld1W::new(self, 31)
    }
}
#[doc = "DR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr1Spec;
impl crate::RegisterSpec for Dr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr1::R`](R) reader structure"]
impl crate::Readable for Dr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dr1::W`](W) writer structure"]
impl crate::Writable for Dr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for Dr1Spec {
    const RESET_VALUE: u32 = 0;
}
