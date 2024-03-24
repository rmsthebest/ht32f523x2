#[doc = "Register `PNSR` reader"]
pub type R = crate::R<PnsrSpec>;
#[doc = "Register `PNSR` writer"]
pub type W = crate::W<PnsrSpec>;
#[doc = "Field `PNSB` reader - PNSB"]
pub type PnsbR = crate::FieldReader<u32>;
#[doc = "Field `PNSB` writer - PNSB"]
pub type PnsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    pub fn pnsb(&self) -> PnsbR {
        PnsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    #[must_use]
    pub fn pnsb(&mut self) -> PnsbW<PnsrSpec> {
        PnsbW::new(self, 0)
    }
}
#[doc = "PNSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pnsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pnsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PnsrSpec;
impl crate::RegisterSpec for PnsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pnsr::R`](R) reader structure"]
impl crate::Readable for PnsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pnsr::W`](W) writer structure"]
impl crate::Writable for PnsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PNSR to value 0"]
impl crate::Resettable for PnsrSpec {
    const RESET_VALUE: u32 = 0;
}
