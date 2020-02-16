#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEN`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SMPSIZE`"]
pub type SMPSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPSIZE`"]
pub struct SMPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BITEXT`"]
pub type BITEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BITEXT`"]
pub struct BITEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> BITEXT_W<'a> {
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
#[doc = "Reader of field `MCLKEN`"]
pub type MCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLKEN`"]
pub struct MCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKEN_W<'a> {
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
#[doc = "Reader of field `REPEAT`"]
pub type REPEAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REPEAT`"]
pub struct REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEAT_W<'a> {
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
#[doc = "Reader of field `CHANNEL`"]
pub type CHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHANNEL`"]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
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
#[doc = "Reader of field `TXMUTE`"]
pub type TXMUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMUTE`"]
pub struct TXMUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMUTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXDMAEN`"]
pub type TXDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMAEN`"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RXDMAEN`"]
pub type RXDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAEN`"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Reader of field `CLKDEN`"]
pub type CLKDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKDEN`"]
pub struct CLKDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDEN_W<'a> {
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
#[doc = "Reader of field `RCEN`"]
pub type RCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCEN`"]
pub struct RCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RCSEL`"]
pub type RCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCSEL`"]
pub struct RCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BCKINV`"]
pub type BCKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCKINV`"]
pub struct BCKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> BCKINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MCKINV`"]
pub type MCKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCKINV`"]
pub struct MCKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    pub fn smpsize(&self) -> SMPSIZE_R {
        SMPSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    pub fn bitext(&self) -> BITEXT_R {
        BITEXT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    pub fn mclken(&self) -> MCLKEN_R {
        MCLKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    pub fn txmute(&self) -> TXMUTE_R {
        TXMUTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    pub fn clkden(&self) -> CLKDEN_R {
        CLKDEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    pub fn rcen(&self) -> RCEN_R {
        RCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    pub fn rcsel(&self) -> RCSEL_R {
        RCSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    pub fn bckinv(&self) -> BCKINV_R {
        BCKINV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    pub fn mckinv(&self) -> MCKINV_R {
        MCKINV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W { w: self }
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    pub fn smpsize(&mut self) -> SMPSIZE_W {
        SMPSIZE_W { w: self }
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    pub fn bitext(&mut self) -> BITEXT_W {
        BITEXT_W { w: self }
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    pub fn mclken(&mut self) -> MCLKEN_W {
        MCLKEN_W { w: self }
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    pub fn repeat(&mut self) -> REPEAT_W {
        REPEAT_W { w: self }
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    pub fn txmute(&mut self) -> TXMUTE_W {
        TXMUTE_W { w: self }
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    pub fn clkden(&mut self) -> CLKDEN_W {
        CLKDEN_W { w: self }
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    pub fn rcen(&mut self) -> RCEN_W {
        RCEN_W { w: self }
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    pub fn rcsel(&mut self) -> RCSEL_W {
        RCSEL_W { w: self }
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    pub fn bckinv(&mut self) -> BCKINV_W {
        BCKINV_W { w: self }
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    pub fn mckinv(&mut self) -> MCKINV_W {
        MCKINV_W { w: self }
    }
}
