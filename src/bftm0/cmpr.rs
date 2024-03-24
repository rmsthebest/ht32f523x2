#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CmprSpec>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CmprSpec>;
#[doc = "Field `CMP` reader - CMP"]
pub type CmpR = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - CMP"]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CmpW<CmprSpec> {
        CmpW::new(self, 0)
    }
}
#[doc = "CMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmprSpec;
impl crate::RegisterSpec for CmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CmprSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CmprSpec {
    const RESET_VALUE: u32 = 0;
}
