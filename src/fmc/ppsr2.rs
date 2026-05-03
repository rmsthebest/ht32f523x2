#[doc = "Register `PPSR2` reader"]
pub type R = crate::R<Ppsr2Spec>;
#[doc = "Register `PPSR2` writer"]
pub type W = crate::W<Ppsr2Spec>;
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
    pub fn ppsb(&mut self) -> PpsbW<'_, Ppsr2Spec> {
        PpsbW::new(self, 0)
    }
}
#[doc = "PPSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ppsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppsr2Spec;
impl crate::RegisterSpec for Ppsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppsr2::R`](R) reader structure"]
impl crate::Readable for Ppsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ppsr2::W`](W) writer structure"]
impl crate::Writable for Ppsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPSR2 to value 0"]
impl crate::Resettable for Ppsr2Spec {}
