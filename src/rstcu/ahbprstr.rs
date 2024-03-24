#[doc = "Register `AHBPRSTR` reader"]
pub type R = crate::R<AhbprstrSpec>;
#[doc = "Register `AHBPRSTR` writer"]
pub type W = crate::W<AhbprstrSpec>;
#[doc = "Field `DMARST` reader - DMARST"]
pub type DmarstR = crate::BitReader;
#[doc = "Field `DMARST` writer - DMARST"]
pub type DmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBIRST` reader - EBIRST"]
pub type EbirstR = crate::BitReader;
#[doc = "Field `EBIRST` writer - EBIRST"]
pub type EbirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRCRST"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRCRST"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARST` reader - PARST"]
pub type ParstR = crate::BitReader;
#[doc = "Field `PARST` writer - PARST"]
pub type ParstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBRST` reader - PBRST"]
pub type PbrstR = crate::BitReader;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRST` reader - PCRST"]
pub type PcrstR = crate::BitReader;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRST` reader - PDRST"]
pub type PdrstR = crate::BitReader;
#[doc = "Field `PDRST` writer - PDRST"]
pub type PdrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&self) -> DmarstR {
        DmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBIRST"]
    #[inline(always)]
    pub fn ebirst(&self) -> EbirstR {
        EbirstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PdrstR {
        PdrstR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    #[must_use]
    pub fn dmarst(&mut self) -> DmarstW<AhbprstrSpec> {
        DmarstW::new(self, 0)
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<AhbprstrSpec> {
        UsbrstW::new(self, 5)
    }
    #[doc = "Bit 6 - EBIRST"]
    #[inline(always)]
    #[must_use]
    pub fn ebirst(&mut self) -> EbirstW<AhbprstrSpec> {
        EbirstW::new(self, 6)
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<AhbprstrSpec> {
        CrcrstW::new(self, 7)
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> ParstW<AhbprstrSpec> {
        ParstW::new(self, 8)
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PbrstW<AhbprstrSpec> {
        PbrstW::new(self, 9)
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PcrstW<AhbprstrSpec> {
        PcrstW::new(self, 10)
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PdrstW<AhbprstrSpec> {
        PdrstW::new(self, 11)
    }
}
#[doc = "AHBPRSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbprstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbprstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbprstrSpec;
impl crate::RegisterSpec for AhbprstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbprstr::R`](R) reader structure"]
impl crate::Readable for AhbprstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbprstr::W`](W) writer structure"]
impl crate::Writable for AhbprstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBPRSTR to value 0"]
impl crate::Resettable for AhbprstrSpec {
    const RESET_VALUE: u32 = 0;
}
