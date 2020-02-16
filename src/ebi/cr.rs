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
#[doc = "Reader of field `MODE0`"]
pub type MODE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE0`"]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `MODE1`"]
pub type MODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE1`"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE2`"]
pub type MODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE2`"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MODE3`"]
pub type MODE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE3`"]
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BANKEN0`"]
pub type BANKEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKEN0`"]
pub struct BANKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKEN0_W<'a> {
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
#[doc = "Reader of field `BANKEN1`"]
pub type BANKEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKEN1`"]
pub struct BANKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKEN1_W<'a> {
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
#[doc = "Reader of field `BANKEN2`"]
pub type BANKEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKEN2`"]
pub struct BANKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKEN2_W<'a> {
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
#[doc = "Reader of field `BANKEN3`"]
pub type BANKEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKEN3`"]
pub struct BANKEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKEN3_W<'a> {
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
#[doc = "Reader of field `NOIDLE0`"]
pub type NOIDLE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE0`"]
pub struct NOIDLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE0_W<'a> {
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
#[doc = "Reader of field `NOIDLE1`"]
pub type NOIDLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE1`"]
pub struct NOIDLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE1_W<'a> {
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
#[doc = "Reader of field `NOIDLE2`"]
pub type NOIDLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE2`"]
pub struct NOIDLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE2_W<'a> {
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
#[doc = "Reader of field `NOIDLE3`"]
pub type NOIDLE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE3`"]
pub struct NOIDLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE3_W<'a> {
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
#[doc = "Reader of field `IDLET`"]
pub type IDLET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDLET`"]
pub struct IDLET_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    pub fn banken0(&self) -> BANKEN0_R {
        BANKEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    pub fn banken1(&self) -> BANKEN1_R {
        BANKEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    pub fn banken2(&self) -> BANKEN2_R {
        BANKEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    pub fn banken3(&self) -> BANKEN3_R {
        BANKEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    pub fn noidle0(&self) -> NOIDLE0_R {
        NOIDLE0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    pub fn noidle1(&self) -> NOIDLE1_R {
        NOIDLE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    pub fn noidle2(&self) -> NOIDLE2_R {
        NOIDLE2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    pub fn noidle3(&self) -> NOIDLE3_R {
        NOIDLE3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    pub fn idlet(&self) -> IDLET_R {
        IDLET_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    pub fn banken0(&mut self) -> BANKEN0_W {
        BANKEN0_W { w: self }
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    pub fn banken1(&mut self) -> BANKEN1_W {
        BANKEN1_W { w: self }
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    pub fn banken2(&mut self) -> BANKEN2_W {
        BANKEN2_W { w: self }
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    pub fn banken3(&mut self) -> BANKEN3_W {
        BANKEN3_W { w: self }
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    pub fn noidle0(&mut self) -> NOIDLE0_W {
        NOIDLE0_W { w: self }
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    pub fn noidle1(&mut self) -> NOIDLE1_W {
        NOIDLE1_W { w: self }
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    pub fn noidle2(&mut self) -> NOIDLE2_W {
        NOIDLE2_W { w: self }
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    pub fn noidle3(&mut self) -> NOIDLE3_W {
        NOIDLE3_W { w: self }
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    pub fn idlet(&mut self) -> IDLET_W {
        IDLET_W { w: self }
    }
}
