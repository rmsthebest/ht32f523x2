#[doc = "Register `APBPRSTR0` reader"]
pub type R = crate::R<Apbprstr0Spec>;
#[doc = "Register `APBPRSTR0` writer"]
pub type W = crate::W<Apbprstr0Spec>;
#[doc = "Field `I2C0RST` reader - I2C0RST"]
pub type I2c0rstR = crate::BitReader;
#[doc = "Field `I2C0RST` writer - I2C0RST"]
pub type I2c0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1RST"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1RST"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0RST` reader - SPI0RST"]
pub type Spi0rstR = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0RST"]
pub type Spi0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR0RST` reader - USR0RST"]
pub type Usr0rstR = crate::BitReader;
#[doc = "Field `USR0RST` writer - USR0RST"]
pub type Usr0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR1RST` reader - USR1RST"]
pub type Usr1rstR = crate::BitReader;
#[doc = "Field `USR1RST` writer - USR1RST"]
pub type Usr1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR0RST` reader - UR0RST"]
pub type Ur0rstR = crate::BitReader;
#[doc = "Field `UR0RST` writer - UR0RST"]
pub type Ur0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR1RST` reader - UR1RST"]
pub type Ur1rstR = crate::BitReader;
#[doc = "Field `UR1RST` writer - UR1RST"]
pub type Ur1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIORST` reader - AFIORST"]
pub type AfiorstR = crate::BitReader;
#[doc = "Field `AFIORST` writer - AFIORST"]
pub type AfiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIRST` reader - EXTIRST"]
pub type ExtirstR = crate::BitReader;
#[doc = "Field `EXTIRST` writer - EXTIRST"]
pub type ExtirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCI0RST` reader - SCI0RST"]
pub type Sci0rstR = crate::BitReader;
#[doc = "Field `SCI0RST` writer - SCI0RST"]
pub type Sci0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SRST` reader - I2SRST"]
pub type I2srstR = crate::BitReader;
#[doc = "Field `I2SRST` writer - I2SRST"]
pub type I2srstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCI1RST` reader - SCI1RST"]
pub type Sci1rstR = crate::BitReader;
#[doc = "Field `SCI1RST` writer - SCI1RST"]
pub type Sci1rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2c0rstR {
        I2c0rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    pub fn usr0rst(&self) -> Usr0rstR {
        Usr0rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    pub fn usr1rst(&self) -> Usr1rstR {
        Usr1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&self) -> Ur0rstR {
        Ur0rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&self) -> Ur1rstR {
        Ur1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AfiorstR {
        AfiorstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&self) -> ExtirstR {
        ExtirstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    pub fn sci0rst(&self) -> Sci0rstR {
        Sci0rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    pub fn i2srst(&self) -> I2srstR {
        I2srstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    pub fn sci1rst(&self) -> Sci1rstR {
        Sci1rstR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0rst(&mut self) -> I2c0rstW<Apbprstr0Spec> {
        I2c0rstW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apbprstr0Spec> {
        I2c1rstW::new(self, 1)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> Spi0rstW<Apbprstr0Spec> {
        Spi0rstW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apbprstr0Spec> {
        Spi1rstW::new(self, 5)
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    #[must_use]
    pub fn usr0rst(&mut self) -> Usr0rstW<Apbprstr0Spec> {
        Usr0rstW::new(self, 8)
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    #[must_use]
    pub fn usr1rst(&mut self) -> Usr1rstW<Apbprstr0Spec> {
        Usr1rstW::new(self, 9)
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur0rst(&mut self) -> Ur0rstW<Apbprstr0Spec> {
        Ur0rstW::new(self, 10)
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur1rst(&mut self) -> Ur1rstW<Apbprstr0Spec> {
        Ur1rstW::new(self, 11)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AfiorstW<Apbprstr0Spec> {
        AfiorstW::new(self, 14)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    #[must_use]
    pub fn extirst(&mut self) -> ExtirstW<Apbprstr0Spec> {
        ExtirstW::new(self, 15)
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    #[must_use]
    pub fn sci0rst(&mut self) -> Sci0rstW<Apbprstr0Spec> {
        Sci0rstW::new(self, 24)
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    #[must_use]
    pub fn i2srst(&mut self) -> I2srstW<Apbprstr0Spec> {
        I2srstW::new(self, 25)
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sci1rst(&mut self) -> Sci1rstW<Apbprstr0Spec> {
        Sci1rstW::new(self, 27)
    }
}
#[doc = "APBPRSTR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbprstr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbprstr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbprstr0Spec;
impl crate::RegisterSpec for Apbprstr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbprstr0::R`](R) reader structure"]
impl crate::Readable for Apbprstr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbprstr0::W`](W) writer structure"]
impl crate::Writable for Apbprstr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBPRSTR0 to value 0"]
impl crate::Resettable for Apbprstr0Spec {
    const RESET_VALUE: u32 = 0;
}
