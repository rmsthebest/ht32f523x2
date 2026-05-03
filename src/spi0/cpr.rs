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
    pub fn cp(&mut self) -> CpW<'_, CprSpec> {
        CpW::new(self, 0)
    }
}
#[doc = "CPR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprSpec;
impl crate::RegisterSpec for CprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpr::R`](R) reader structure"]
impl crate::Readable for CprSpec {}
#[doc = "`write(|w| ..)` method takes [`cpr::W`](W) writer structure"]
impl crate::Writable for CprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPR to value 0"]
impl crate::Resettable for CprSpec {}
