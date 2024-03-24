#[doc = "Register `WCR` reader"]
pub type R = crate::R<WcrSpec>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
#[doc = "Field `ADWLE` reader - ADWLE"]
pub type AdwleR = crate::BitReader;
#[doc = "Field `ADWLE` writer - ADWLE"]
pub type AdwleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWUE` reader - ADWUE"]
pub type AdwueR = crate::BitReader;
#[doc = "Field `ADWUE` writer - ADWUE"]
pub type AdwueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWALL` reader - ADWALL"]
pub type AdwallR = crate::BitReader;
#[doc = "Field `ADWALL` writer - ADWALL"]
pub type AdwallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWCH` reader - ADWCH"]
pub type AdwchR = crate::FieldReader;
#[doc = "Field `ADWCH` writer - ADWCH"]
pub type AdwchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADLCH` reader - ADLCH"]
pub type AdlchR = crate::FieldReader;
#[doc = "Field `ADLCH` writer - ADLCH"]
pub type AdlchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADUCH` reader - ADUCH"]
pub type AduchR = crate::FieldReader;
#[doc = "Field `ADUCH` writer - ADUCH"]
pub type AduchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    pub fn adwle(&self) -> AdwleR {
        AdwleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    pub fn adwue(&self) -> AdwueR {
        AdwueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    pub fn adwall(&self) -> AdwallR {
        AdwallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    pub fn adwch(&self) -> AdwchR {
        AdwchR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    pub fn adlch(&self) -> AdlchR {
        AdlchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    pub fn aduch(&self) -> AduchR {
        AduchR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    #[must_use]
    pub fn adwle(&mut self) -> AdwleW<WcrSpec> {
        AdwleW::new(self, 0)
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    #[must_use]
    pub fn adwue(&mut self) -> AdwueW<WcrSpec> {
        AdwueW::new(self, 1)
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    #[must_use]
    pub fn adwall(&mut self) -> AdwallW<WcrSpec> {
        AdwallW::new(self, 2)
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    #[must_use]
    pub fn adwch(&mut self) -> AdwchW<WcrSpec> {
        AdwchW::new(self, 8)
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    #[must_use]
    pub fn adlch(&mut self) -> AdlchW<WcrSpec> {
        AdlchW::new(self, 16)
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    #[must_use]
    pub fn aduch(&mut self) -> AduchW<WcrSpec> {
        AduchW::new(self, 24)
    }
}
#[doc = "WCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WcrSpec {
    const RESET_VALUE: u32 = 0;
}
