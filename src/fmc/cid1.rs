#[doc = "Register `CID1` reader"]
pub type R = crate::R<Cid1Spec>;
#[doc = "Register `CID1` writer"]
pub type W = crate::W<Cid1Spec>;
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
    pub fn cid(&mut self) -> CidW<Cid1Spec> {
        CidW::new(self, 0)
    }
}
#[doc = "CID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`write(|w| ..)` method takes [`cid1::W`](W) writer structure"]
impl crate::Writable for Cid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for Cid1Spec {
    const RESET_VALUE: u32 = 0;
}
