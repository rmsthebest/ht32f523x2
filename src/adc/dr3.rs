#[doc = "Register `DR3` reader"]
pub type R = crate::R<Dr3Spec>;
#[doc = "Register `DR3` writer"]
pub type W = crate::W<Dr3Spec>;
#[doc = "Field `ADD3` reader - ADD3"]
pub type Add3R = crate::FieldReader<u16>;
#[doc = "Field `ADD3` writer - ADD3"]
pub type Add3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD3` reader - ADVLD3"]
pub type Advld3R = crate::BitReader;
#[doc = "Field `ADVLD3` writer - ADVLD3"]
pub type Advld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    pub fn add3(&self) -> Add3R {
        Add3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    pub fn advld3(&self) -> Advld3R {
        Advld3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    #[must_use]
    pub fn add3(&mut self) -> Add3W<Dr3Spec> {
        Add3W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    #[must_use]
    pub fn advld3(&mut self) -> Advld3W<Dr3Spec> {
        Advld3W::new(self, 31)
    }
}
#[doc = "DR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr3Spec;
impl crate::RegisterSpec for Dr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for Dr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dr3::W`](W) writer structure"]
impl crate::Writable for Dr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for Dr3Spec {
    const RESET_VALUE: u32 = 0;
}
