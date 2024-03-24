#[doc = "Register `CPR` reader"]
pub type R = crate::R<CprSpec>;
#[doc = "Register `CPR` writer"]
pub type W = crate::W<CprSpec>;
#[doc = "Field `CP` reader - CP"]
pub type CpR = crate::FieldReader<u16>;
#[doc = "Field `CP` writer - CP"]
pub type CpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CpW<CprSpec> {
        CpW::new(self, 0)
    }
}
#[doc = "CPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprSpec;
impl crate::RegisterSpec for CprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpr::R`](R) reader structure"]
impl crate::Readable for CprSpec {}
#[doc = "`write(|w| ..)` method takes [`cpr::W`](W) writer structure"]
impl crate::Writable for CprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPR to value 0"]
impl crate::Resettable for CprSpec {
    const RESET_VALUE: u32 = 0;
}
