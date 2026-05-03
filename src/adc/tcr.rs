#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `ADSW` reader - ADSW"]
pub type AdswR = crate::BitReader;
#[doc = "Field `ADSW` writer - ADSW"]
pub type AdswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEXTI` reader - ADEXTI"]
pub type AdextiR = crate::BitReader;
#[doc = "Field `ADEXTI` writer - ADEXTI"]
pub type AdextiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - TM"]
pub type TmR = crate::BitReader;
#[doc = "Field `TM` writer - TM"]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFTM` reader - BFTM"]
pub type BftmR = crate::BitReader;
#[doc = "Field `BFTM` writer - BFTM"]
pub type BftmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` reader - CMP"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CMP` writer - CMP"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> AdswR {
        AdswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> AdextiR {
        AdextiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    pub fn bftm(&self) -> BftmR {
        BftmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&mut self) -> AdswW<'_, TcrSpec> {
        AdswW::new(self, 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&mut self) -> AdextiW<'_, TcrSpec> {
        AdextiW::new(self, 1)
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<'_, TcrSpec> {
        TmW::new(self, 2)
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    pub fn bftm(&mut self) -> BftmW<'_, TcrSpec> {
        BftmW::new(self, 3)
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, TcrSpec> {
        CmpW::new(self, 4)
    }
}
#[doc = "TCR\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
