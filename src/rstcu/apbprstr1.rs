#[doc = "Register `APBPRSTR1` reader"]
pub type R = crate::R<Apbprstr1Spec>;
#[doc = "Register `APBPRSTR1` writer"]
pub type W = crate::W<Apbprstr1Spec>;
#[doc = "Field `MCTM0RST` reader - MCTM0RST"]
pub type Mctm0rstR = crate::BitReader;
#[doc = "Field `MCTM0RST` writer - MCTM0RST"]
pub type Mctm0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTRST` reader - WDTRST"]
pub type WdtrstR = crate::BitReader;
#[doc = "Field `WDTRST` writer - WDTRST"]
pub type WdtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTM0RST` reader - GPTM0RST"]
pub type Gptm0rstR = crate::BitReader;
#[doc = "Field `GPTM0RST` writer - GPTM0RST"]
pub type Gptm0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTM1RST` reader - GPTM1RST"]
pub type Gptm1rstR = crate::BitReader;
#[doc = "Field `GPTM1RST` writer - GPTM1RST"]
pub type Gptm1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFTM0RST` reader - BFTM0RST"]
pub type Bftm0rstR = crate::BitReader;
#[doc = "Field `BFTM0RST` writer - BFTM0RST"]
pub type Bftm0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFTM1RST` reader - BFTM1RST"]
pub type Bftm1rstR = crate::BitReader;
#[doc = "Field `BFTM1RST` writer - BFTM1RST"]
pub type Bftm1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRST` reader - CMPRST"]
pub type CmprstR = crate::BitReader;
#[doc = "Field `CMPRST` writer - CMPRST"]
pub type CmprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADCRST"]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADCRST"]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTM0RST` reader - SCTM0RST"]
pub type Sctm0rstR = crate::BitReader;
#[doc = "Field `SCTM0RST` writer - SCTM0RST"]
pub type Sctm0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTM1RST` reader - SCTM1RST"]
pub type Sctm1rstR = crate::BitReader;
#[doc = "Field `SCTM1RST` writer - SCTM1RST"]
pub type Sctm1rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    pub fn mctm0rst(&self) -> Mctm0rstR {
        Mctm0rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&self) -> WdtrstR {
        WdtrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    pub fn gptm0rst(&self) -> Gptm0rstR {
        Gptm0rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    pub fn gptm1rst(&self) -> Gptm1rstR {
        Gptm1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    pub fn bftm0rst(&self) -> Bftm0rstR {
        Bftm0rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    pub fn bftm1rst(&self) -> Bftm1rstR {
        Bftm1rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    pub fn cmprst(&self) -> CmprstR {
        CmprstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    pub fn sctm0rst(&self) -> Sctm0rstR {
        Sctm0rstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    pub fn sctm1rst(&self) -> Sctm1rstR {
        Sctm1rstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn mctm0rst(&mut self) -> Mctm0rstW<Apbprstr1Spec> {
        Mctm0rstW::new(self, 0)
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrst(&mut self) -> WdtrstW<Apbprstr1Spec> {
        WdtrstW::new(self, 4)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0rst(&mut self) -> Gptm0rstW<Apbprstr1Spec> {
        Gptm0rstW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1rst(&mut self) -> Gptm1rstW<Apbprstr1Spec> {
        Gptm1rstW::new(self, 9)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0rst(&mut self) -> Bftm0rstW<Apbprstr1Spec> {
        Bftm0rstW::new(self, 16)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1rst(&mut self) -> Bftm1rstW<Apbprstr1Spec> {
        Bftm1rstW::new(self, 17)
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    #[must_use]
    pub fn cmprst(&mut self) -> CmprstW<Apbprstr1Spec> {
        CmprstW::new(self, 22)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<Apbprstr1Spec> {
        AdcrstW::new(self, 24)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0rst(&mut self) -> Sctm0rstW<Apbprstr1Spec> {
        Sctm0rstW::new(self, 28)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1rst(&mut self) -> Sctm1rstW<Apbprstr1Spec> {
        Sctm1rstW::new(self, 29)
    }
}
#[doc = "APBPRSTR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbprstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbprstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbprstr1Spec;
impl crate::RegisterSpec for Apbprstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbprstr1::R`](R) reader structure"]
impl crate::Readable for Apbprstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbprstr1::W`](W) writer structure"]
impl crate::Writable for Apbprstr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBPRSTR1 to value 0"]
impl crate::Resettable for Apbprstr1Spec {
    const RESET_VALUE: u32 = 0;
}
