#[doc = "Register `PSCR` reader"]
pub type R = crate::R<PscrSpec>;
#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PscrSpec>;
#[doc = "Field `PSCV` reader - PSCV"]
pub type PscvR = crate::FieldReader<u16>;
#[doc = "Field `PSCV` writer - PSCV"]
pub type PscvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    pub fn pscv(&self) -> PscvR {
        PscvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    pub fn pscv(&mut self) -> PscvW<'_, PscrSpec> {
        PscvW::new(self, 0)
    }
}
#[doc = "PSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrSpec;
impl crate::RegisterSpec for PscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscr::R`](R) reader structure"]
impl crate::Readable for PscrSpec {}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PscrSpec {}
