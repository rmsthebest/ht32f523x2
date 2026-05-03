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
    pub fn cmp(&mut self) -> CmpW<'_, CmprSpec> {
        CmpW::new(self, 0)
    }
}
#[doc = "CMPR\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmprSpec;
impl crate::RegisterSpec for CmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CmprSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CmprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CmprSpec {}
