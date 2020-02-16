#[doc = "Reader of register APBPRSTR0"]
pub type R = crate::R<u32, super::APBPRSTR0>;
#[doc = "Writer for register APBPRSTR0"]
pub type W = crate::W<u32, super::APBPRSTR0>;
#[doc = "Register APBPRSTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APBPRSTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0RST`"]
pub type I2C0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0RST`"]
pub struct I2C0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `I2C1RST`"]
pub type I2C1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1RST`"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SPI0RST`"]
pub type SPI0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0RST`"]
pub struct SPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI1RST`"]
pub type SPI1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1RST`"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `USR0RST`"]
pub type USR0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR0RST`"]
pub struct USR0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USR0RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `USR1RST`"]
pub type USR1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR1RST`"]
pub struct USR1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USR1RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UR0RST`"]
pub type UR0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR0RST`"]
pub struct UR0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UR0RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UR1RST`"]
pub type UR1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR1RST`"]
pub struct UR1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UR1RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AFIORST`"]
pub type AFIORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AFIORST`"]
pub struct AFIORST_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIORST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EXTIRST`"]
pub type EXTIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIRST`"]
pub struct EXTIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SCI0RST`"]
pub type SCI0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCI0RST`"]
pub struct SCI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCI0RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2SRST`"]
pub type I2SRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2SRST`"]
pub struct I2SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SCI1RST`"]
pub type SCI1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCI1RST`"]
pub struct SCI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCI1RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    pub fn usr0rst(&self) -> USR0RST_R {
        USR0RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    pub fn usr1rst(&self) -> USR1RST_R {
        USR1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&self) -> UR0RST_R {
        UR0RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&self) -> UR1RST_R {
        UR1RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&self) -> EXTIRST_R {
        EXTIRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    pub fn sci0rst(&self) -> SCI0RST_R {
        SCI0RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    pub fn i2srst(&self) -> I2SRST_R {
        I2SRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    pub fn sci1rst(&self) -> SCI1RST_R {
        SCI1RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2C0RST_W {
        I2C0RST_W { w: self }
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W { w: self }
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    pub fn usr0rst(&mut self) -> USR0RST_W {
        USR0RST_W { w: self }
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    pub fn usr1rst(&mut self) -> USR1RST_W {
        USR1RST_W { w: self }
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&mut self) -> UR0RST_W {
        UR0RST_W { w: self }
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&mut self) -> UR1RST_W {
        UR1RST_W { w: self }
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&mut self) -> AFIORST_W {
        AFIORST_W { w: self }
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&mut self) -> EXTIRST_W {
        EXTIRST_W { w: self }
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    pub fn sci0rst(&mut self) -> SCI0RST_W {
        SCI0RST_W { w: self }
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    pub fn i2srst(&mut self) -> I2SRST_W {
        I2SRST_W { w: self }
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    pub fn sci1rst(&mut self) -> SCI1RST_W {
        SCI1RST_W { w: self }
    }
}
