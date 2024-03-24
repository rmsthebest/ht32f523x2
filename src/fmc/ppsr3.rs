#[doc = "Register `PPSR3` reader"]
pub type R = crate::R<Ppsr3Spec>;
#[doc = "Register `PPSR3` writer"]
pub type W = crate::W<Ppsr3Spec>;
#[doc = "Field `PPSB` reader - PPSB"]
pub type PpsbR = crate::FieldReader<u32>;
#[doc = "Field `PPSB` writer - PPSB"]
pub type PpsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSB"]
    #[inline(always)]
    pub fn ppsb(&self) -> PpsbR {
        PpsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSB"]
    #[inline(always)]
    #[must_use]
    pub fn ppsb(&mut self) -> PpsbW<Ppsr3Spec> {
        PpsbW::new(self, 0)
    }
}
#[doc = "PPSR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppsr3Spec;
impl crate::RegisterSpec for Ppsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppsr3::R`](R) reader structure"]
impl crate::Readable for Ppsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ppsr3::W`](W) writer structure"]
impl crate::Writable for Ppsr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPSR3 to value 0"]
impl crate::Resettable for Ppsr3Spec {
    const RESET_VALUE: u32 = 0;
}
