#[doc = "Reader of register APBCCR0"]
pub type R = crate::R<u32, super::APBCCR0>;
#[doc = "Writer for register APBCCR0"]
pub type W = crate::W<u32, super::APBCCR0>;
#[doc = "Register APBCCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APBCCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0EN`"]
pub type I2C0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0EN`"]
pub struct I2C0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0EN_W<'a> {
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
#[doc = "Reader of field `I2C1EN`"]
pub type I2C1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1EN`"]
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
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
#[doc = "Reader of field `SPI0EN`"]
pub type SPI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0EN`"]
pub struct SPI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0EN_W<'a> {
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
#[doc = "Reader of field `SPI1EN`"]
pub type SPI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EN`"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
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
#[doc = "Reader of field `USR0EN`"]
pub type USR0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR0EN`"]
pub struct USR0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR0EN_W<'a> {
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
#[doc = "Reader of field `USR1EN`"]
pub type USR1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR1EN`"]
pub struct USR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR1EN_W<'a> {
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
#[doc = "Reader of field `UR0EN`"]
pub type UR0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR0EN`"]
pub struct UR0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UR0EN_W<'a> {
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
#[doc = "Reader of field `UR1EN`"]
pub type UR1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR1EN`"]
pub struct UR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UR1EN_W<'a> {
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
#[doc = "Reader of field `AFIOEN`"]
pub type AFIOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AFIOEN`"]
pub struct AFIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIOEN_W<'a> {
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
#[doc = "Reader of field `EXTIEN`"]
pub type EXTIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIEN`"]
pub struct EXTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIEN_W<'a> {
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
#[doc = "Reader of field `SCI0EN`"]
pub type SCI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCI0EN`"]
pub struct SCI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCI0EN_W<'a> {
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
#[doc = "Reader of field `I2SEN`"]
pub type I2SEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2SEN`"]
pub struct I2SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SEN_W<'a> {
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
#[doc = "Reader of field `SCI1EN`"]
pub type SCI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCI1EN`"]
pub struct SCI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCI1EN_W<'a> {
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
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USR0EN"]
    #[inline(always)]
    pub fn usr0en(&self) -> USR0EN_R {
        USR0EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USR1EN"]
    #[inline(always)]
    pub fn usr1en(&self) -> USR1EN_R {
        USR1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&self) -> UR0EN_R {
        UR0EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&self) -> UR1EN_R {
        UR1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&self) -> EXTIEN_R {
        EXTIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCI0EN"]
    #[inline(always)]
    pub fn sci0en(&self) -> SCI0EN_R {
        SCI0EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SCI1EN"]
    #[inline(always)]
    pub fn sci1en(&self) -> SCI1EN_R {
        SCI1EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2C0EN_W {
        I2C0EN_W { w: self }
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> SPI0EN_W {
        SPI0EN_W { w: self }
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 8 - USR0EN"]
    #[inline(always)]
    pub fn usr0en(&mut self) -> USR0EN_W {
        USR0EN_W { w: self }
    }
    #[doc = "Bit 9 - USR1EN"]
    #[inline(always)]
    pub fn usr1en(&mut self) -> USR1EN_W {
        USR1EN_W { w: self }
    }
    #[doc = "Bit 10 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&mut self) -> UR0EN_W {
        UR0EN_W { w: self }
    }
    #[doc = "Bit 11 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&mut self) -> UR1EN_W {
        UR1EN_W { w: self }
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W {
        AFIOEN_W { w: self }
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&mut self) -> EXTIEN_W {
        EXTIEN_W { w: self }
    }
    #[doc = "Bit 24 - SCI0EN"]
    #[inline(always)]
    pub fn sci0en(&mut self) -> SCI0EN_W {
        SCI0EN_W { w: self }
    }
    #[doc = "Bit 25 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W { w: self }
    }
    #[doc = "Bit 27 - SCI1EN"]
    #[inline(always)]
    pub fn sci1en(&mut self) -> SCI1EN_W {
        SCI1EN_W { w: self }
    }
}
