#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADMODE` reader - ADMODE"]
pub type AdmodeR = crate::FieldReader;
#[doc = "Field `ADMODE` writer - ADMODE"]
pub type AdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCRST` reader - ADCRST"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADCRST"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADCEN"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADCEN"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSEQL` reader - ADSEQL"]
pub type AdseqlR = crate::FieldReader;
#[doc = "Field `ADSEQL` writer - ADSEQL"]
pub type AdseqlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADSUBL` reader - ADSUBL"]
pub type AdsublR = crate::FieldReader;
#[doc = "Field `ADSUBL` writer - ADSUBL"]
pub type AdsublW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> AdmodeR {
        AdmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&self) -> AdseqlR {
        AdseqlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&self) -> AdsublR {
        AdsublR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&mut self) -> AdmodeW<'_, CrSpec> {
        AdmodeW::new(self, 0)
    }
    #[doc = "Bit 6 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, CrSpec> {
        AdcrstW::new(self, 6)
    }
    #[doc = "Bit 7 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, CrSpec> {
        AdcenW::new(self, 7)
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&mut self) -> AdseqlW<'_, CrSpec> {
        AdseqlW::new(self, 8)
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&mut self) -> AdsublW<'_, CrSpec> {
        AdsublW::new(self, 16)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
