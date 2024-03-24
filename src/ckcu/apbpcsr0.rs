#[doc = "Register `APBPCSR0` reader"]
pub type R = crate::R<Apbpcsr0Spec>;
#[doc = "Register `APBPCSR0` writer"]
pub type W = crate::W<Apbpcsr0Spec>;
#[doc = "Field `I2C0PCLK` reader - I2C0PCLK"]
pub type I2c0pclkR = crate::FieldReader;
#[doc = "Field `I2C0PCLK` writer - I2C0PCLK"]
pub type I2c0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1PCLK` reader - I2C1PCLK"]
pub type I2c1pclkR = crate::FieldReader;
#[doc = "Field `I2C1PCLK` writer - I2C1PCLK"]
pub type I2c1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI0PCLK` reader - SPI0PCLK"]
pub type Spi0pclkR = crate::FieldReader;
#[doc = "Field `SPI0PCLK` writer - SPI0PCLK"]
pub type Spi0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI1PCLK` reader - SPI1PCLK"]
pub type Spi1pclkR = crate::FieldReader;
#[doc = "Field `SPI1PCLK` writer - SPI1PCLK"]
pub type Spi1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BFTM0PCLK` reader - BFTM0PCLK"]
pub type Bftm0pclkR = crate::FieldReader;
#[doc = "Field `BFTM0PCLK` writer - BFTM0PCLK"]
pub type Bftm0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BFTM1PCLK` reader - BFTM1PCLK"]
pub type Bftm1pclkR = crate::FieldReader;
#[doc = "Field `BFTM1PCLK` writer - BFTM1PCLK"]
pub type Bftm1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCTM0PCLK` reader - MCTM0PCLK"]
pub type Mctm0pclkR = crate::FieldReader;
#[doc = "Field `MCTM0PCLK` writer - MCTM0PCLK"]
pub type Mctm0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPTM0PCLK` reader - GPTM0PCLK"]
pub type Gptm0pclkR = crate::FieldReader;
#[doc = "Field `GPTM0PCLK` writer - GPTM0PCLK"]
pub type Gptm0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPTM1PCLK` reader - GPTM1PCLK"]
pub type Gptm1pclkR = crate::FieldReader;
#[doc = "Field `GPTM1PCLK` writer - GPTM1PCLK"]
pub type Gptm1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USR0PCLK` reader - USR0PCLK"]
pub type Usr0pclkR = crate::FieldReader;
#[doc = "Field `USR0PCLK` writer - USR0PCLK"]
pub type Usr0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USR1PCLK` reader - USR1PCLK"]
pub type Usr1pclkR = crate::FieldReader;
#[doc = "Field `USR1PCLK` writer - USR1PCLK"]
pub type Usr1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UR0PCLK` reader - UR0PCLK"]
pub type Ur0pclkR = crate::FieldReader;
#[doc = "Field `UR0PCLK` writer - UR0PCLK"]
pub type Ur0pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UR1PCLK` reader - UR1PCLK"]
pub type Ur1pclkR = crate::FieldReader;
#[doc = "Field `UR1PCLK` writer - UR1PCLK"]
pub type Ur1pclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    pub fn i2c0pclk(&self) -> I2c0pclkR {
        I2c0pclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    pub fn i2c1pclk(&self) -> I2c1pclkR {
        I2c1pclkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    pub fn spi0pclk(&self) -> Spi0pclkR {
        Spi0pclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    pub fn spi1pclk(&self) -> Spi1pclkR {
        Spi1pclkR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    pub fn bftm0pclk(&self) -> Bftm0pclkR {
        Bftm0pclkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    pub fn bftm1pclk(&self) -> Bftm1pclkR {
        Bftm1pclkR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    pub fn mctm0pclk(&self) -> Mctm0pclkR {
        Mctm0pclkR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    pub fn gptm0pclk(&self) -> Gptm0pclkR {
        Gptm0pclkR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    pub fn gptm1pclk(&self) -> Gptm1pclkR {
        Gptm1pclkR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    pub fn usr0pclk(&self) -> Usr0pclkR {
        Usr0pclkR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    pub fn usr1pclk(&self) -> Usr1pclkR {
        Usr1pclkR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    pub fn ur0pclk(&self) -> Ur0pclkR {
        Ur0pclkR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    pub fn ur1pclk(&self) -> Ur1pclkR {
        Ur1pclkR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0pclk(&mut self) -> I2c0pclkW<Apbpcsr0Spec> {
        I2c0pclkW::new(self, 0)
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1pclk(&mut self) -> I2c1pclkW<Apbpcsr0Spec> {
        I2c1pclkW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi0pclk(&mut self) -> Spi0pclkW<Apbpcsr0Spec> {
        Spi0pclkW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi1pclk(&mut self) -> Spi1pclkW<Apbpcsr0Spec> {
        Spi1pclkW::new(self, 6)
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0pclk(&mut self) -> Bftm0pclkW<Apbpcsr0Spec> {
        Bftm0pclkW::new(self, 12)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1pclk(&mut self) -> Bftm1pclkW<Apbpcsr0Spec> {
        Bftm1pclkW::new(self, 14)
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn mctm0pclk(&mut self) -> Mctm0pclkW<Apbpcsr0Spec> {
        Mctm0pclkW::new(self, 16)
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0pclk(&mut self) -> Gptm0pclkW<Apbpcsr0Spec> {
        Gptm0pclkW::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1pclk(&mut self) -> Gptm1pclkW<Apbpcsr0Spec> {
        Gptm1pclkW::new(self, 22)
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn usr0pclk(&mut self) -> Usr0pclkW<Apbpcsr0Spec> {
        Usr0pclkW::new(self, 24)
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn usr1pclk(&mut self) -> Usr1pclkW<Apbpcsr0Spec> {
        Usr1pclkW::new(self, 26)
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur0pclk(&mut self) -> Ur0pclkW<Apbpcsr0Spec> {
        Ur0pclkW::new(self, 28)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur1pclk(&mut self) -> Ur1pclkW<Apbpcsr0Spec> {
        Ur1pclkW::new(self, 30)
    }
}
#[doc = "APBPCSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbpcsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbpcsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbpcsr0Spec;
impl crate::RegisterSpec for Apbpcsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbpcsr0::R`](R) reader structure"]
impl crate::Readable for Apbpcsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbpcsr0::W`](W) writer structure"]
impl crate::Writable for Apbpcsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBPCSR0 to value 0"]
impl crate::Resettable for Apbpcsr0Spec {
    const RESET_VALUE: u32 = 0;
}
