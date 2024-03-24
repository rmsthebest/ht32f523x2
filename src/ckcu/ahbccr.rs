#[doc = "Register `AHBCCR` reader"]
pub type R = crate::R<AhbccrSpec>;
#[doc = "Register `AHBCCR` writer"]
pub type W = crate::W<AhbccrSpec>;
#[doc = "Field `FMCEN` reader - FMCEN"]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMCEN"]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAMEN"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAMEN"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMAEN` reader - PDMAEN"]
pub type PdmaenR = crate::BitReader;
#[doc = "Field `PDMAEN` writer - PDMAEN"]
pub type PdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMEN` reader - BMEN"]
pub type BmenR = crate::BitReader;
#[doc = "Field `BMEN` writer - BMEN"]
pub type BmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBEN` reader - APBEN"]
pub type ApbenR = crate::BitReader;
#[doc = "Field `APBEN` writer - APBEN"]
pub type ApbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - USBEN"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USBEN"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKREFEN` reader - CKREFEN"]
pub type CkrefenR = crate::BitReader;
#[doc = "Field `CKREFEN` writer - CKREFEN"]
pub type CkrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBIEN` reader - EBIEN"]
pub type EbienR = crate::BitReader;
#[doc = "Field `EBIEN` writer - EBIEN"]
pub type EbienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRCEN"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRCEN"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAEN` reader - PAEN"]
pub type PaenR = crate::BitReader;
#[doc = "Field `PAEN` writer - PAEN"]
pub type PaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBEN` reader - PBEN"]
pub type PbenR = crate::BitReader;
#[doc = "Field `PBEN` writer - PBEN"]
pub type PbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - PCEN"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - PCEN"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - PDEN"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - PDEN"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PdmaenR {
        PdmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BmenR {
        BmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    pub fn apben(&self) -> ApbenR {
        ApbenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    pub fn ckrefen(&self) -> CkrefenR {
        CkrefenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    pub fn ebien(&self) -> EbienR {
        EbienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PaenR {
        PaenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PbenR {
        PbenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FmcenW<AhbccrSpec> {
        FmcenW::new(self, 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SramenW<AhbccrSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pdmaen(&mut self) -> PdmaenW<AhbccrSpec> {
        PdmaenW::new(self, 4)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    #[must_use]
    pub fn bmen(&mut self) -> BmenW<AhbccrSpec> {
        BmenW::new(self, 5)
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    #[must_use]
    pub fn apben(&mut self) -> ApbenW<AhbccrSpec> {
        ApbenW::new(self, 6)
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> UsbenW<AhbccrSpec> {
        UsbenW::new(self, 10)
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckrefen(&mut self) -> CkrefenW<AhbccrSpec> {
        CkrefenW::new(self, 11)
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ebien(&mut self) -> EbienW<AhbccrSpec> {
        EbienW::new(self, 12)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<AhbccrSpec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PaenW<AhbccrSpec> {
        PaenW::new(self, 16)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PbenW<AhbccrSpec> {
        PbenW::new(self, 17)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<AhbccrSpec> {
        PcenW::new(self, 18)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PdenW<AhbccrSpec> {
        PdenW::new(self, 19)
    }
}
#[doc = "AHBCCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbccrSpec;
impl crate::RegisterSpec for AhbccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbccr::R`](R) reader structure"]
impl crate::Readable for AhbccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbccr::W`](W) writer structure"]
impl crate::Writable for AhbccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBCCR to value 0"]
impl crate::Resettable for AhbccrSpec {
    const RESET_VALUE: u32 = 0;
}
