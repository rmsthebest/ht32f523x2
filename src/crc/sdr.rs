#[doc = "Register `SDR` reader"]
pub type R = crate::R<SdrSpec>;
#[doc = "Register `SDR` writer"]
pub type W = crate::W<SdrSpec>;
#[doc = "Field `SEED` reader - SEED"]
pub type SeedR = crate::FieldReader<u32>;
#[doc = "Field `SEED` writer - SEED"]
pub type SeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&self) -> SeedR {
        SeedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&mut self) -> SeedW<'_, SdrSpec> {
        SeedW::new(self, 0)
    }
}
#[doc = "SDR\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrSpec;
impl crate::RegisterSpec for SdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr::R`](R) reader structure"]
impl crate::Readable for SdrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdr::W`](W) writer structure"]
impl crate::Writable for SdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SdrSpec {}
