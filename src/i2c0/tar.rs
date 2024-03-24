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
    #[must_use]
    pub fn tar(&mut self) -> TarW<TarSpec> {
        TarW::new(self, 0)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    #[must_use]
    pub fn rwd(&mut self) -> RwdW<TarSpec> {
        RwdW::new(self, 10)
    }
}
#[doc = "TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAR to value 0"]
impl crate::Resettable for TarSpec {
    const RESET_VALUE: u32 = 0;
}
