#[doc = "Register `OPCR` reader"]
pub type R = crate::R<OpcrSpec>;
#[doc = "Register `OPCR` writer"]
pub type W = crate::W<OpcrSpec>;
#[doc = "Field `OPM` reader - OPM"]
pub type OpmR = crate::FieldReader;
#[doc = "Field `OPM` writer - OPM"]
pub type OpmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<'_, OpcrSpec> {
        OpmW::new(self, 1)
    }
}
#[doc = "OPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`opcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpcrSpec;
impl crate::RegisterSpec for OpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcr::R`](R) reader structure"]
impl crate::Readable for OpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`opcr::W`](W) writer structure"]
impl crate::Writable for OpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCR to value 0"]
impl crate::Resettable for OpcrSpec {}
