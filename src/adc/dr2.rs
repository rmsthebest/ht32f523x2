#[doc = "Register `DR2` reader"]
pub type R = crate::R<Dr2Spec>;
#[doc = "Register `DR2` writer"]
pub type W = crate::W<Dr2Spec>;
#[doc = "Field `ADD2` reader - ADD2"]
pub type Add2R = crate::FieldReader<u16>;
#[doc = "Field `ADD2` writer - ADD2"]
pub type Add2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD2` reader - ADVLD2"]
pub type Advld2R = crate::BitReader;
#[doc = "Field `ADVLD2` writer - ADVLD2"]
pub type Advld2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    pub fn add2(&self) -> Add2R {
        Add2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    pub fn advld2(&self) -> Advld2R {
        Advld2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    #[must_use]
    pub fn add2(&mut self) -> Add2W<Dr2Spec> {
        Add2W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    #[must_use]
    pub fn advld2(&mut self) -> Advld2W<Dr2Spec> {
        Advld2W::new(self, 31)
    }
}
#[doc = "DR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr2Spec;
impl crate::RegisterSpec for Dr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr2::R`](R) reader structure"]
impl crate::Readable for Dr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dr2::W`](W) writer structure"]
impl crate::Writable for Dr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR2 to value 0"]
impl crate::Resettable for Dr2Spec {
    const RESET_VALUE: u32 = 0;
}
