#[doc = "Register `APBCCR0` reader"]
pub type R = crate::R<Apbccr0Spec>;
#[doc = "Register `APBCCR0` writer"]
pub type W = crate::W<Apbccr0Spec>;
#[doc = "Field `I2C0EN` reader - I2C0EN"]
pub type I2c0enR = crate::BitReader;
#[doc = "Field `I2C0EN` writer - I2C0EN"]
pub type I2c0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1EN"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1EN"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0EN` reader - SPI0EN"]
pub type Spi0enR = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0EN"]
pub type Spi0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR0EN` reader - USR0EN"]
pub type Usr0enR = crate::BitReader;
#[doc = "Field `USR0EN` writer - USR0EN"]
pub type Usr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR1EN` reader - USR1EN"]
pub type Usr1enR = crate::BitReader;
#[doc = "Field `USR1EN` writer - USR1EN"]
pub type Usr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR0EN` reader - UR0EN"]
pub type Ur0enR = crate::BitReader;
#[doc = "Field `UR0EN` writer - UR0EN"]
pub type Ur0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR1EN` reader - UR1EN"]
pub type Ur1enR = crate::BitReader;
#[doc = "Field `UR1EN` writer - UR1EN"]
pub type Ur1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AfioenR = crate::BitReader;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AfioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIEN` reader - EXTIEN"]
pub type ExtienR = crate::BitReader;
#[doc = "Field `EXTIEN` writer - EXTIEN"]
pub type ExtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCI0EN` reader - SCI0EN"]
pub type Sci0enR = crate::BitReader;
#[doc = "Field `SCI0EN` writer - SCI0EN"]
pub type Sci0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SEN` reader - I2SEN"]
pub type I2senR = crate::BitReader;
#[doc = "Field `I2SEN` writer - I2SEN"]
pub type I2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCI1EN` reader - SCI1EN"]
pub type Sci1enR = crate::BitReader;
#[doc = "Field `SCI1EN` writer - SCI1EN"]
pub type Sci1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2c0enR {
        I2c0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&self) -> Spi0enR {
        Spi0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USR0EN"]
    #[inline(always)]
    pub fn usr0en(&self) -> Usr0enR {
        Usr0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USR1EN"]
    #[inline(always)]
    pub fn usr1en(&self) -> Usr1enR {
        Usr1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&self) -> Ur0enR {
        Ur0enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&self) -> Ur1enR {
        Ur1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AfioenR {
        AfioenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&self) -> ExtienR {
        ExtienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - SCI0EN"]
    #[inline(always)]
    pub fn sci0en(&self) -> Sci0enR {
        Sci0enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2senR {
        I2senR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - SCI1EN"]
    #[inline(always)]
    pub fn sci1en(&self) -> Sci1enR {
        Sci1enR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0en(&mut self) -> I2c0enW<Apbccr0Spec> {
        I2c0enW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2c1enW<Apbccr0Spec> {
        I2c1enW::new(self, 1)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> Spi0enW<Apbccr0Spec> {
        Spi0enW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<Apbccr0Spec> {
        Spi1enW::new(self, 5)
    }
    #[doc = "Bit 8 - USR0EN"]
    #[inline(always)]
    #[must_use]
    pub fn usr0en(&mut self) -> Usr0enW<Apbccr0Spec> {
        Usr0enW::new(self, 8)
    }
    #[doc = "Bit 9 - USR1EN"]
    #[inline(always)]
    #[must_use]
    pub fn usr1en(&mut self) -> Usr1enW<Apbccr0Spec> {
        Usr1enW::new(self, 9)
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur0en(&mut self) -> Ur0enW<Apbccr0Spec> {
        Ur0enW::new(self, 10)
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ur1en(&mut self) -> Ur1enW<Apbccr0Spec> {
        Ur1enW::new(self, 11)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AfioenW<Apbccr0Spec> {
        AfioenW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    #[must_use]
    pub fn extien(&mut self) -> ExtienW<Apbccr0Spec> {
        ExtienW::new(self, 15)
    }
    #[doc = "Bit 24 - SCI0EN"]
    #[inline(always)]
    #[must_use]
    pub fn sci0en(&mut self) -> Sci0enW<Apbccr0Spec> {
        Sci0enW::new(self, 24)
    }
    #[doc = "Bit 25 - I2SEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2senW<Apbccr0Spec> {
        I2senW::new(self, 25)
    }
    #[doc = "Bit 27 - SCI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sci1en(&mut self) -> Sci1enW<Apbccr0Spec> {
        Sci1enW::new(self, 27)
    }
}
#[doc = "APBCCR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbccr0Spec;
impl crate::RegisterSpec for Apbccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbccr0::R`](R) reader structure"]
impl crate::Readable for Apbccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbccr0::W`](W) writer structure"]
impl crate::Writable for Apbccr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBCCR0 to value 0"]
impl crate::Resettable for Apbccr0Spec {
    const RESET_VALUE: u32 = 0;
}
