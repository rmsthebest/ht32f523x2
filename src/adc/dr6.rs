#[doc = "Register `DR6` reader"]
pub type R = crate::R<Dr6Spec>;
#[doc = "Register `DR6` writer"]
pub type W = crate::W<Dr6Spec>;
#[doc = "Field `ADD6` reader - ADD6"]
pub type Add6R = crate::FieldReader<u16>;
#[doc = "Field `ADD6` writer - ADD6"]
pub type Add6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADVLD6` reader - ADVLD6"]
pub type Advld6R = crate::BitReader;
#[doc = "Field `ADVLD6` writer - ADVLD6"]
pub type Advld6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    pub fn add6(&self) -> Add6R {
        Add6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    pub fn advld6(&self) -> Advld6R {
        Advld6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    pub fn add6(&mut self) -> Add6W<'_, Dr6Spec> {
        Add6W::new(self, 0)
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    pub fn advld6(&mut self) -> Advld6W<'_, Dr6Spec> {
        Advld6W::new(self, 31)
    }
}
#[doc = "DR6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr6Spec;
impl crate::RegisterSpec for Dr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr6::R`](R) reader structure"]
impl crate::Readable for Dr6Spec {}
#[doc = "`write(|w| ..)` method takes [`dr6::W`](W) writer structure"]
impl crate::Writable for Dr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR6 to value 0"]
impl crate::Resettable for Dr6Spec {}
