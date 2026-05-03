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
    pub fn cid(&mut self) -> CidW<'_, Cid1Spec> {
        CidW::new(self, 0)
    }
}
#[doc = "CID1\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`write(|w| ..)` method takes [`cid1::W`](W) writer structure"]
impl crate::Writable for Cid1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for Cid1Spec {}
