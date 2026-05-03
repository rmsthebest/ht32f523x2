#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CHKSUM` reader - CHKSUM"]
pub type ChksumR = crate::FieldReader<u32>;
#[doc = "Field `CHKSUM` writer - CHKSUM"]
pub type ChksumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    pub fn chksum(&self) -> ChksumR {
        ChksumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    pub fn chksum(&mut self) -> ChksumW<'_, CsrSpec> {
        ChksumW::new(self, 0)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
