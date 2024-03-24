#[doc = "Register `VALR0` reader"]
pub type R = crate::R<Valr0Spec>;
#[doc = "Register `VALR0` writer"]
pub type W = crate::W<Valr0Spec>;
#[doc = "Field `CVRVAL` reader - CVRVAL"]
pub type CvrvalR = crate::FieldReader;
#[doc = "Field `CVRVAL` writer - CVRVAL"]
pub type CvrvalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    pub fn cvrval(&self) -> CvrvalR {
        CvrvalR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    #[must_use]
    pub fn cvrval(&mut self) -> CvrvalW<Valr0Spec> {
        CvrvalW::new(self, 0)
    }
}
#[doc = "VALR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`valr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`valr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Valr0Spec;
impl crate::RegisterSpec for Valr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`valr0::R`](R) reader structure"]
impl crate::Readable for Valr0Spec {}
#[doc = "`write(|w| ..)` method takes [`valr0::W`](W) writer structure"]
impl crate::Writable for Valr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALR0 to value 0"]
impl crate::Resettable for Valr0Spec {
    const RESET_VALUE: u32 = 0;
}
