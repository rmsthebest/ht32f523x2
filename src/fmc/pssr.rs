#[doc = "Register `PSSR` reader"]
pub type R = crate::R<PssrSpec>;
#[doc = "Register `PSSR` writer"]
pub type W = crate::W<PssrSpec>;
#[doc = "Field `PSSB` reader - PSSB"]
pub type PssbR = crate::FieldReader<u32>;
#[doc = "Field `PSSB` writer - PSSB"]
pub type PssbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    pub fn pssb(&self) -> PssbR {
        PssbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    #[must_use]
    pub fn pssb(&mut self) -> PssbW<PssrSpec> {
        PssbW::new(self, 0)
    }
}
#[doc = "PSSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PssrSpec;
impl crate::RegisterSpec for PssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssr::R`](R) reader structure"]
impl crate::Readable for PssrSpec {}
#[doc = "`write(|w| ..)` method takes [`pssr::W`](W) writer structure"]
impl crate::Writable for PssrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSSR to value 0"]
impl crate::Resettable for PssrSpec {
    const RESET_VALUE: u32 = 0;
}
