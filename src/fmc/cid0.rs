#[doc = "Register `CID0` reader"]
pub type R = crate::R<Cid0Spec>;
#[doc = "Register `CID0` writer"]
pub type W = crate::W<Cid0Spec>;
#[doc = "Field `CID` reader - CID"]
pub type CidR = crate::FieldReader<u32>;
#[doc = "Field `CID` writer - CID"]
pub type CidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CID"]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CID"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CidW<Cid0Spec> {
        CidW::new(self, 0)
    }
}
#[doc = "CID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid0Spec;
impl crate::RegisterSpec for Cid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid0::R`](R) reader structure"]
impl crate::Readable for Cid0Spec {}
#[doc = "`write(|w| ..)` method takes [`cid0::W`](W) writer structure"]
impl crate::Writable for Cid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CID0 to value 0"]
impl crate::Resettable for Cid0Spec {
    const RESET_VALUE: u32 = 0;
}
