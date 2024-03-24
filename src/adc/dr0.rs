#[doc = "Register `DR0` reader"]
pub type R = crate::R<Dr0Spec>;
#[doc = "Register `DR0` writer"]
pub type W = crate::W<Dr0Spec>;
#[doc = "Field `ADD0` reader - ADD0"]
pub type Add0R = crate::FieldReader<u16>;
#[doc = "Field `ADD0` writer - ADD0"]
pub type Add0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD0` reader - ADVLD0"]
pub type Advld0R = crate::BitReader;
#[doc = "Field `ADVLD0` writer - ADVLD0"]
pub type Advld0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    pub fn add0(&self) -> Add0R {
        Add0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    pub fn advld0(&self) -> Advld0R {
        Advld0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> Add0W<Dr0Spec> {
        Add0W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    #[must_use]
    pub fn advld0(&mut self) -> Advld0W<Dr0Spec> {
        Advld0W::new(self, 31)
    }
}
#[doc = "DR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr0Spec;
impl crate::RegisterSpec for Dr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for Dr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dr0::W`](W) writer structure"]
impl crate::Writable for Dr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for Dr0Spec {
    const RESET_VALUE: u32 = 0;
}
