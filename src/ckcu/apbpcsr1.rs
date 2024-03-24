#[doc = "Register `APBPCSR1` reader"]
pub type R = crate::R<Apbpcsr1Spec>;
#[doc = "Register `APBPCSR1` writer"]
pub type W = crate::W<Apbpcsr1Spec>;
#[doc = "Field `AFIOPCLK` reader - AFIOPCLK"]
pub type AfiopclkR = crate::FieldReader;
#[doc = "Field `AFIOPCLK` writer - AFIOPCLK"]
pub type AfiopclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTIPCLK` reader - EXTIPCLK"]
pub type ExtipclkR = crate::FieldReader;
#[doc = "Field `EXTIPCLK` writer - EXTIPCLK"]
pub type ExtipclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCCPCLK` reader - ADCCPCLK"]
pub type AdccpclkR = crate::FieldReader;
#[doc = "Field `ADCCPCLK` writer - ADCCPCLK"]
pub type AdccpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPPCLK` reader - CMPPCLK"]
pub type CmppclkR = crate::FieldReader;
#[doc = "Field `CMPPCLK` writer - CMPPCLK"]
pub type CmppclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDTRPCLK` reader - WDTRPCLK"]
pub type WdtrpclkR = crate::FieldReader;
#[doc = "Field `WDTRPCLK` writer - WDTRPCLK"]
pub type WdtrpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BKPRPCLK` reader - BKPRPCLK"]
pub type BkprpclkR = crate::FieldReader;
#[doc = "Field `BKPRPCLK` writer - BKPRPCLK"]
pub type BkprpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCI0PCLK` reader - SCI0PCLK"]
pub type Sci0pclkR = crate::FieldReader;
#[doc = "Field `SCI0PCLK` writer - SCI0PCLK"]
pub type Sci0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCI1PCLK` reader - SCI1PCLK"]
pub type Sci1pclkR = crate::FieldReader;
#[doc = "Field `SCI1PCLK` writer - SCI1PCLK"]
pub type Sci1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SPCLK` reader - I2SPCLK"]
pub type I2spclkR = crate::FieldReader;
#[doc = "Field `I2SPCLK` writer - I2SPCLK"]
pub type I2spclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCTM0PCLK` reader - SCTM0PCLK"]
pub type Sctm0pclkR = crate::FieldReader;
#[doc = "Field `SCTM0PCLK` writer - SCTM0PCLK"]
pub type Sctm0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCTM1PCLK` reader - SCTM1PCLK"]
pub type Sctm1pclkR = crate::FieldReader;
#[doc = "Field `SCTM1PCLK` writer - SCTM1PCLK"]
pub type Sctm1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    pub fn afiopclk(&self) -> AfiopclkR {
        AfiopclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    pub fn extipclk(&self) -> ExtipclkR {
        ExtipclkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    pub fn adccpclk(&self) -> AdccpclkR {
        AdccpclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    pub fn cmppclk(&self) -> CmppclkR {
        CmppclkR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    pub fn wdtrpclk(&self) -> WdtrpclkR {
        WdtrpclkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    pub fn bkprpclk(&self) -> BkprpclkR {
        BkprpclkR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SCI0PCLK"]
    #[inline(always)]
    pub fn sci0pclk(&self) -> Sci0pclkR {
        Sci0pclkR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SCI1PCLK"]
    #[inline(always)]
    pub fn sci1pclk(&self) -> Sci1pclkR {
        Sci1pclkR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    pub fn i2spclk(&self) -> I2spclkR {
        I2spclkR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline(always)]
    pub fn sctm0pclk(&self) -> Sctm0pclkR {
        Sctm0pclkR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline(always)]
    pub fn sctm1pclk(&self) -> Sctm1pclkR {
        Sctm1pclkR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn afiopclk(&mut self) -> AfiopclkW<Apbpcsr1Spec> {
        AfiopclkW::new(self, 0)
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn extipclk(&mut self) -> ExtipclkW<Apbpcsr1Spec> {
        ExtipclkW::new(self, 2)
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn adccpclk(&mut self) -> AdccpclkW<Apbpcsr1Spec> {
        AdccpclkW::new(self, 4)
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn cmppclk(&mut self) -> CmppclkW<Apbpcsr1Spec> {
        CmppclkW::new(self, 8)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrpclk(&mut self) -> WdtrpclkW<Apbpcsr1Spec> {
        WdtrpclkW::new(self, 12)
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bkprpclk(&mut self) -> BkprpclkW<Apbpcsr1Spec> {
        BkprpclkW::new(self, 14)
    }
    #[doc = "Bits 16:17 - SCI0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sci0pclk(&mut self) -> Sci0pclkW<Apbpcsr1Spec> {
        Sci0pclkW::new(self, 16)
    }
    #[doc = "Bits 18:19 - SCI1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sci1pclk(&mut self) -> Sci1pclkW<Apbpcsr1Spec> {
        Sci1pclkW::new(self, 18)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2spclk(&mut self) -> I2spclkW<Apbpcsr1Spec> {
        I2spclkW::new(self, 20)
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0pclk(&mut self) -> Sctm0pclkW<Apbpcsr1Spec> {
        Sctm0pclkW::new(self, 24)
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1pclk(&mut self) -> Sctm1pclkW<Apbpcsr1Spec> {
        Sctm1pclkW::new(self, 26)
    }
}
#[doc = "APBPCSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbpcsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbpcsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbpcsr1Spec;
impl crate::RegisterSpec for Apbpcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbpcsr1::R`](R) reader structure"]
impl crate::Readable for Apbpcsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbpcsr1::W`](W) writer structure"]
impl crate::Writable for Apbpcsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBPCSR1 to value 0"]
impl crate::Resettable for Apbpcsr1Spec {
    const RESET_VALUE: u32 = 0;
}
