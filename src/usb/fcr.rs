#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `FRNUM` reader - FRNUM"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FRNUM` writer - FRNUM"]
pub type FrnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SOFLCK` reader - SOFLCK"]
pub type SoflckR = crate::BitReader;
#[doc = "Field `SOFLCK` writer - SOFLCK"]
pub type SoflckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSOF` reader - LSOF"]
pub type LsofR = crate::FieldReader;
#[doc = "Field `LSOF` writer - LSOF"]
pub type LsofW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    pub fn soflck(&self) -> SoflckR {
        SoflckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LsofR {
        LsofR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    #[must_use]
    pub fn frnum(&mut self) -> FrnumW<FcrSpec> {
        FrnumW::new(self, 0)
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    #[must_use]
    pub fn soflck(&mut self) -> SoflckW<FcrSpec> {
        SoflckW::new(self, 16)
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    #[must_use]
    pub fn lsof(&mut self) -> LsofW<FcrSpec> {
        LsofW::new(self, 17)
    }
}
#[doc = "FCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u32 = 0;
}
