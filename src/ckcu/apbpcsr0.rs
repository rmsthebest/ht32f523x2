#[doc = "Reader of register APBPCSR0"]
pub type R = crate::R<u32, super::APBPCSR0>;
#[doc = "Writer for register APBPCSR0"]
pub type W = crate::W<u32, super::APBPCSR0>;
#[doc = "Register APBPCSR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APBPCSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0PCLK`"]
pub type I2C0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C0PCLK`"]
pub struct I2C0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `I2C1PCLK`"]
pub type I2C1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1PCLK`"]
pub struct I2C1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI0PCLK`"]
pub type SPI0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0PCLK`"]
pub struct SPI0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI1PCLK`"]
pub type SPI1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1PCLK`"]
pub struct SPI1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BFTM0PCLK`"]
pub type BFTM0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BFTM0PCLK`"]
pub struct BFTM0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `BFTM1PCLK`"]
pub type BFTM1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BFTM1PCLK`"]
pub struct BFTM1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCTM0PCLK`"]
pub type MCTM0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCTM0PCLK`"]
pub struct MCTM0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTM0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPTM0PCLK`"]
pub type GPTM0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTM0PCLK`"]
pub struct GPTM0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `GPTM1PCLK`"]
pub type GPTM1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTM1PCLK`"]
pub struct GPTM1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `USR0PCLK`"]
pub type USR0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR0PCLK`"]
pub struct USR0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USR0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `USR1PCLK`"]
pub type USR1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR1PCLK`"]
pub struct USR1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USR1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `UR0PCLK`"]
pub type UR0PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UR0PCLK`"]
pub struct UR0PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UR0PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `UR1PCLK`"]
pub type UR1PCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UR1PCLK`"]
pub struct UR1PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UR1PCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    pub fn i2c0pclk(&self) -> I2C0PCLK_R {
        I2C0PCLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    pub fn i2c1pclk(&self) -> I2C1PCLK_R {
        I2C1PCLK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    pub fn spi0pclk(&self) -> SPI0PCLK_R {
        SPI0PCLK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    pub fn spi1pclk(&self) -> SPI1PCLK_R {
        SPI1PCLK_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    pub fn bftm0pclk(&self) -> BFTM0PCLK_R {
        BFTM0PCLK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    pub fn bftm1pclk(&self) -> BFTM1PCLK_R {
        BFTM1PCLK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    pub fn mctm0pclk(&self) -> MCTM0PCLK_R {
        MCTM0PCLK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    pub fn gptm0pclk(&self) -> GPTM0PCLK_R {
        GPTM0PCLK_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    pub fn gptm1pclk(&self) -> GPTM1PCLK_R {
        GPTM1PCLK_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    pub fn usr0pclk(&self) -> USR0PCLK_R {
        USR0PCLK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    pub fn usr1pclk(&self) -> USR1PCLK_R {
        USR1PCLK_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    pub fn ur0pclk(&self) -> UR0PCLK_R {
        UR0PCLK_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    pub fn ur1pclk(&self) -> UR1PCLK_R {
        UR1PCLK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    pub fn i2c0pclk(&mut self) -> I2C0PCLK_W {
        I2C0PCLK_W { w: self }
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    pub fn i2c1pclk(&mut self) -> I2C1PCLK_W {
        I2C1PCLK_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    pub fn spi0pclk(&mut self) -> SPI0PCLK_W {
        SPI0PCLK_W { w: self }
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    pub fn spi1pclk(&mut self) -> SPI1PCLK_W {
        SPI1PCLK_W { w: self }
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    pub fn bftm0pclk(&mut self) -> BFTM0PCLK_W {
        BFTM0PCLK_W { w: self }
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    pub fn bftm1pclk(&mut self) -> BFTM1PCLK_W {
        BFTM1PCLK_W { w: self }
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    pub fn mctm0pclk(&mut self) -> MCTM0PCLK_W {
        MCTM0PCLK_W { w: self }
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    pub fn gptm0pclk(&mut self) -> GPTM0PCLK_W {
        GPTM0PCLK_W { w: self }
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    pub fn gptm1pclk(&mut self) -> GPTM1PCLK_W {
        GPTM1PCLK_W { w: self }
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    pub fn usr0pclk(&mut self) -> USR0PCLK_W {
        USR0PCLK_W { w: self }
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    pub fn usr1pclk(&mut self) -> USR1PCLK_W {
        USR1PCLK_W { w: self }
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    pub fn ur0pclk(&mut self) -> UR0PCLK_W {
        UR0PCLK_W { w: self }
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    pub fn ur1pclk(&mut self) -> UR1PCLK_W {
        UR1PCLK_W { w: self }
    }
}
