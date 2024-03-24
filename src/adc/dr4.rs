#[doc = "Register `DR4` reader"]
pub type R = crate::R<Dr4Spec>;
#[doc = "Register `DR4` writer"]
pub type W = crate::W<Dr4Spec>;
#[doc = "Field `ADD4` reader - ADD4"]
pub type Add4R = crate::FieldReader<u16>;
#[doc = "Field `ADD4` writer - ADD4"]
pub type Add4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD4` reader - ADVLD4"]
pub type Advld4R = crate::BitReader;
#[doc = "Field `ADVLD4` writer - ADVLD4"]
pub type Advld4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    pub fn add4(&self) -> Add4R {
        Add4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    pub fn advld4(&self) -> Advld4R {
        Advld4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    #[must_use]
    pub fn add4(&mut self) -> Add4W<Dr4Spec> {
        Add4W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    #[must_use]
    pub fn advld4(&mut self) -> Advld4W<Dr4Spec> {
        Advld4W::new(self, 31)
    }
}
#[doc = "DR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr4Spec;
impl crate::RegisterSpec for Dr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr4::R`](R) reader structure"]
impl crate::Readable for Dr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dr4::W`](W) writer structure"]
impl crate::Writable for Dr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR4 to value 0"]
impl crate::Resettable for Dr4Spec {
    const RESET_VALUE: u32 = 0;
}
