#[doc = "Register `TSR` reader"]
pub type R = crate::R<TsrSpec>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TsrSpec>;
#[doc = "Field `ADSC` reader - ADSC"]
pub type AdscR = crate::BitReader;
#[doc = "Field `ADSC` writer - ADSC"]
pub type AdscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEXTIS` reader - ADEXTIS"]
pub type AdextisR = crate::FieldReader;
#[doc = "Field `ADEXTIS` writer - ADEXTIS"]
pub type AdextisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMS` reader - TMS"]
pub type TmsR = crate::FieldReader;
#[doc = "Field `TMS` writer - TMS"]
pub type TmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BFTMS` reader - BFTMS"]
pub type BftmsR = crate::BitReader;
#[doc = "Field `BFTMS` writer - BFTMS"]
pub type BftmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPS` reader - CMPS"]
pub type CmpsR = crate::BitReader;
#[doc = "Field `CMPS` writer - CMPS"]
pub type CmpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TME` reader - TME"]
pub type TmeR = crate::FieldReader;
#[doc = "Field `TME` writer - TME"]
pub type TmeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> AdscR {
        AdscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> AdextisR {
        AdextisR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    pub fn tms(&self) -> TmsR {
        TmsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    pub fn bftms(&self) -> BftmsR {
        BftmsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    pub fn cmps(&self) -> CmpsR {
        CmpsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TmeR {
        TmeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&mut self) -> AdscW<'_, TsrSpec> {
        AdscW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&mut self) -> AdextisW<'_, TsrSpec> {
        AdextisW::new(self, 8)
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    pub fn tms(&mut self) -> TmsW<'_, TsrSpec> {
        TmsW::new(self, 16)
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    pub fn bftms(&mut self) -> BftmsW<'_, TsrSpec> {
        BftmsW::new(self, 19)
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    pub fn cmps(&mut self) -> CmpsW<'_, TsrSpec> {
        CmpsW::new(self, 20)
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    pub fn tme(&mut self) -> TmeW<'_, TsrSpec> {
        TmeW::new(self, 24)
    }
}
#[doc = "TSR\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsrSpec;
impl crate::RegisterSpec for TsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TsrSpec {}
