#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `CONV` reader - CONV"]
pub type ConvR = crate::BitReader;
#[doc = "Field `CONV` writer - CONV"]
pub type ConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CREP` reader - CREP"]
pub type CrepR = crate::BitReader;
#[doc = "Field `CREP` writer - CREP"]
pub type CrepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTEN` reader - WTEN"]
pub type WtenR = crate::BitReader;
#[doc = "Field `WTEN` writer - WTEN"]
pub type WtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIM` reader - SCIM"]
pub type ScimR = crate::BitReader;
#[doc = "Field `SCIM` writer - SCIM"]
pub type ScimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY` reader - RETRY"]
pub type RetryR = crate::BitReader;
#[doc = "Field `RETRY` writer - RETRY"]
pub type RetryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSCI` reader - ENSCI"]
pub type EnsciR = crate::BitReader;
#[doc = "Field `ENSCI` writer - ENSCI"]
pub type EnsciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETCNF` reader - DETCNF"]
pub type DetcnfR = crate::BitReader;
#[doc = "Field `DETCNF` writer - DETCNF"]
pub type DetcnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA` reader - TXDMA"]
pub type TxdmaR = crate::BitReader;
#[doc = "Field `TXDMA` writer - TXDMA"]
pub type TxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMA` reader - RXDMA"]
pub type RxdmaR = crate::BitReader;
#[doc = "Field `RXDMA` writer - RXDMA"]
pub type RxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    pub fn conv(&self) -> ConvR {
        ConvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    pub fn crep(&self) -> CrepR {
        CrepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    pub fn wten(&self) -> WtenR {
        WtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    pub fn scim(&self) -> ScimR {
        ScimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    pub fn retry(&self) -> RetryR {
        RetryR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    pub fn ensci(&self) -> EnsciR {
        EnsciR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    pub fn detcnf(&self) -> DetcnfR {
        DetcnfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TxdmaR {
        TxdmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    pub fn rxdma(&self) -> RxdmaR {
        RxdmaR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    #[must_use]
    pub fn conv(&mut self) -> ConvW<CrSpec> {
        ConvW::new(self, 0)
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CrepW<CrSpec> {
        CrepW::new(self, 1)
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wten(&mut self) -> WtenW<CrSpec> {
        WtenW::new(self, 2)
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    #[must_use]
    pub fn scim(&mut self) -> ScimW<CrSpec> {
        ScimW::new(self, 3)
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RetryW<CrSpec> {
        RetryW::new(self, 4)
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    #[must_use]
    pub fn ensci(&mut self) -> EnsciW<CrSpec> {
        EnsciW::new(self, 5)
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    #[must_use]
    pub fn detcnf(&mut self) -> DetcnfW<CrSpec> {
        DetcnfW::new(self, 6)
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TxdmaW<CrSpec> {
        TxdmaW::new(self, 8)
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RxdmaW<CrSpec> {
        RxdmaW::new(self, 9)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
