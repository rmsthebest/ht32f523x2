#[doc = "Register `PLLCR` reader"]
pub type R = crate::R<PllcrSpec>;
#[doc = "Register `PLLCR` writer"]
pub type W = crate::W<PllcrSpec>;
#[doc = "Field `PLLBPS` reader - PLLBPS"]
pub type PllbpsR = crate::BitReader;
#[doc = "Field `PLLBPS` writer - PLLBPS"]
pub type PllbpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    pub fn pllbps(&self) -> PllbpsR {
        PllbpsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    #[must_use]
    pub fn pllbps(&mut self) -> PllbpsW<PllcrSpec> {
        PllbpsW::new(self, 31)
    }
}
#[doc = "PLLCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcrSpec;
impl crate::RegisterSpec for PllcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcr::R`](R) reader structure"]
impl crate::Readable for PllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcr::W`](W) writer structure"]
impl crate::Writable for PllcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCR to value 0"]
impl crate::Resettable for PllcrSpec {
    const RESET_VALUE: u32 = 0;
}
