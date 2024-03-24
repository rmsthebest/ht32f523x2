#[doc = "Register `PPSR1` reader"]
pub type R = crate::R<Ppsr1Spec>;
#[doc = "Register `PPSR1` writer"]
pub type W = crate::W<Ppsr1Spec>;
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
    pub fn ppsb(&mut self) -> PpsbW<Ppsr1Spec> {
        PpsbW::new(self, 0)
    }
}
#[doc = "PPSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppsr1Spec;
impl crate::RegisterSpec for Ppsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppsr1::R`](R) reader structure"]
impl crate::Readable for Ppsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ppsr1::W`](W) writer structure"]
impl crate::Writable for Ppsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPSR1 to value 0"]
impl crate::Resettable for Ppsr1Spec {
    const RESET_VALUE: u32 = 0;
}
