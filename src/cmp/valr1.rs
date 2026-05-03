#[doc = "Register `VALR1` reader"]
pub type R = crate::R<Valr1Spec>;
#[doc = "Register `VALR1` writer"]
pub type W = crate::W<Valr1Spec>;
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
    pub fn cvrval(&mut self) -> CvrvalW<'_, Valr1Spec> {
        CvrvalW::new(self, 0)
    }
}
#[doc = "VALR1\n\nYou can [`read`](crate::Reg::read) this register and get [`valr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`valr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Valr1Spec;
impl crate::RegisterSpec for Valr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`valr1::R`](R) reader structure"]
impl crate::Readable for Valr1Spec {}
#[doc = "`write(|w| ..)` method takes [`valr1::W`](W) writer structure"]
impl crate::Writable for Valr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALR1 to value 0"]
impl crate::Resettable for Valr1Spec {}
