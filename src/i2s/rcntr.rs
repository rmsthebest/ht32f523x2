#[doc = "Register `RCNTR` reader"]
pub type R = crate::R<RcntrSpec>;
#[doc = "Register `RCNTR` writer"]
pub type W = crate::W<RcntrSpec>;
#[doc = "Field `RCNTR` reader - RCNTR"]
pub type RcntrR = crate::FieldReader<u32>;
#[doc = "Field `RCNTR` writer - RCNTR"]
pub type RcntrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    pub fn rcntr(&self) -> RcntrR {
        RcntrR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    #[must_use]
    pub fn rcntr(&mut self) -> RcntrW<RcntrSpec> {
        RcntrW::new(self, 0)
    }
}
#[doc = "RCNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcntrSpec;
impl crate::RegisterSpec for RcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcntr::R`](R) reader structure"]
impl crate::Readable for RcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcntr::W`](W) writer structure"]
impl crate::Writable for RcntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCNTR to value 0"]
impl crate::Resettable for RcntrSpec {
    const RESET_VALUE: u32 = 0;
}
