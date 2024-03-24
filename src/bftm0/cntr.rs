#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CntrSpec>;
#[doc = "Field `CNTR` reader - CNTR"]
pub type CntrR = crate::FieldReader<u32>;
#[doc = "Field `CNTR` writer - CNTR"]
pub type CntrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTR"]
    #[inline(always)]
    pub fn cntr(&self) -> CntrR {
        CntrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTR"]
    #[inline(always)]
    #[must_use]
    pub fn cntr(&mut self) -> CntrW<CntrSpec> {
        CntrW::new(self, 0)
    }
}
#[doc = "CNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CntrSpec {
    const RESET_VALUE: u32 = 0;
}
