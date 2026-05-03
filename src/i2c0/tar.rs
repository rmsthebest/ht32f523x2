#[doc = "Register `TAR` reader"]
pub type R = crate::R<TarSpec>;
#[doc = "Register `TAR` writer"]
pub type W = crate::W<TarSpec>;
#[doc = "Field `TAR` reader - TAR"]
pub type TarR = crate::FieldReader<u16>;
#[doc = "Field `TAR` writer - TAR"]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RWD` reader - RWD"]
pub type RwdR = crate::BitReader;
#[doc = "Field `RWD` writer - RWD"]
pub type RwdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    pub fn rwd(&self) -> RwdR {
        RwdR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TarW<'_, TarSpec> {
        TarW::new(self, 0)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    pub fn rwd(&mut self) -> RwdW<'_, TarSpec> {
        RwdW::new(self, 10)
    }
}
#[doc = "TAR\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAR to value 0"]
impl crate::Resettable for TarSpec {}
