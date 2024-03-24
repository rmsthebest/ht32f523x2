#[doc = "Register `CID2` reader"]
pub type R = crate::R<Cid2Spec>;
#[doc = "Register `CID2` writer"]
pub type W = crate::W<Cid2Spec>;
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
    pub fn cid(&mut self) -> CidW<Cid2Spec> {
        CidW::new(self, 0)
    }
}
#[doc = "CID2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid2Spec;
impl crate::RegisterSpec for Cid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for Cid2Spec {}
#[doc = "`write(|w| ..)` method takes [`cid2::W`](W) writer structure"]
impl crate::Writable for Cid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for Cid2Spec {
    const RESET_VALUE: u32 = 0;
}
