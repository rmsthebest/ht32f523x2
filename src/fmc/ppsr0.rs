#[doc = "Register `PPSR0` reader"]
pub type R = crate::R<Ppsr0Spec>;
#[doc = "Register `PPSR0` writer"]
pub type W = crate::W<Ppsr0Spec>;
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
    pub fn ppsb(&mut self) -> PpsbW<Ppsr0Spec> {
        PpsbW::new(self, 0)
    }
}
#[doc = "PPSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppsr0Spec;
impl crate::RegisterSpec for Ppsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppsr0::R`](R) reader structure"]
impl crate::Readable for Ppsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ppsr0::W`](W) writer structure"]
impl crate::Writable for Ppsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPSR0 to value 0"]
impl crate::Resettable for Ppsr0Spec {
    const RESET_VALUE: u32 = 0;
}
