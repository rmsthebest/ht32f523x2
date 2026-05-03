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
    pub fn rcntr(&mut self) -> RcntrW<'_, RcntrSpec> {
        RcntrW::new(self, 0)
    }
}
#[doc = "RCNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcntrSpec;
impl crate::RegisterSpec for RcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcntr::R`](R) reader structure"]
impl crate::Readable for RcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcntr::W`](W) writer structure"]
impl crate::Writable for RcntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCNTR to value 0"]
impl crate::Resettable for RcntrSpec {}
