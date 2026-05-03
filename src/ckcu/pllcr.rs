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
    pub fn pllbps(&mut self) -> PllbpsW<'_, PllcrSpec> {
        PllbpsW::new(self, 31)
    }
}
#[doc = "PLLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcrSpec;
impl crate::RegisterSpec for PllcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcr::R`](R) reader structure"]
impl crate::Readable for PllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcr::W`](W) writer structure"]
impl crate::Writable for PllcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCR to value 0"]
impl crate::Resettable for PllcrSpec {}
