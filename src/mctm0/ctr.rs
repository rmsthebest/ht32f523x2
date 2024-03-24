#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `TME` reader - TME"]
pub type TmeR = crate::BitReader;
#[doc = "Field `TME` writer - TME"]
pub type TmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRBE` reader - CRBE"]
pub type CrbeR = crate::BitReader;
#[doc = "Field `CRBE` writer - CRBE"]
pub type CrbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRE` reader - COMPRE"]
pub type CompreR = crate::BitReader;
#[doc = "Field `COMPRE` writer - COMPRE"]
pub type CompreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMUS` reader - COMUS"]
pub type ComusR = crate::BitReader;
#[doc = "Field `COMUS` writer - COMUS"]
pub type ComusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHCCDS` reader - CHCCDS"]
pub type ChccdsR = crate::BitReader;
#[doc = "Field `CHCCDS` writer - CHCCDS"]
pub type ChccdsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TmeR {
        TmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&self) -> CrbeR {
        CrbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - COMPRE"]
    #[inline(always)]
    pub fn compre(&self) -> CompreR {
        CompreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - COMUS"]
    #[inline(always)]
    pub fn comus(&self) -> ComusR {
        ComusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&self) -> ChccdsR {
        ChccdsR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TmeW<CtrSpec> {
        TmeW::new(self, 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    #[must_use]
    pub fn crbe(&mut self) -> CrbeW<CtrSpec> {
        CrbeW::new(self, 1)
    }
    #[doc = "Bit 8 - COMPRE"]
    #[inline(always)]
    #[must_use]
    pub fn compre(&mut self) -> CompreW<CtrSpec> {
        CompreW::new(self, 8)
    }
    #[doc = "Bit 9 - COMUS"]
    #[inline(always)]
    #[must_use]
    pub fn comus(&mut self) -> ComusW<CtrSpec> {
        ComusW::new(self, 9)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    #[must_use]
    pub fn chccds(&mut self) -> ChccdsW<CtrSpec> {
        ChccdsW::new(self, 16)
    }
}
#[doc = "CTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0;
}
