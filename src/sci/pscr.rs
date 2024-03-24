#[doc = "Register `PSCR` reader"]
pub type R = crate::R<PscrSpec>;
#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PscrSpec>;
#[doc = "Field `PSC` reader - PSC"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - PSC"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<PscrSpec> {
        PscW::new(self, 0)
    }
}
#[doc = "PSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrSpec;
impl crate::RegisterSpec for PscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscr::R`](R) reader structure"]
impl crate::Readable for PscrSpec {}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PscrSpec {
    const RESET_VALUE: u32 = 0;
}
