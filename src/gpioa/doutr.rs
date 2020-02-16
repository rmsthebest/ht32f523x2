#[doc = "Reader of register DOUTR"]
pub type R = crate::R<u32, super::DOUTR>;
#[doc = "Writer for register DOUTR"]
pub type W = crate::W<u32, super::DOUTR>;
#[doc = "Register DOUTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT0`"]
pub type DOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT0`"]
pub struct DOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_W<'a> {
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
#[doc = "Reader of field `DOUT1`"]
pub type DOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT1`"]
pub struct DOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_W<'a> {
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
#[doc = "Reader of field `DOUT2`"]
pub type DOUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT2`"]
pub struct DOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_W<'a> {
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
#[doc = "Reader of field `DOUT3`"]
pub type DOUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT3`"]
pub struct DOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DOUT4`"]
pub type DOUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT4`"]
pub struct DOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_W<'a> {
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
#[doc = "Reader of field `DOUT5`"]
pub type DOUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT5`"]
pub struct DOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_W<'a> {
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
#[doc = "Reader of field `DOUT6`"]
pub type DOUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT6`"]
pub struct DOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DOUT7`"]
pub type DOUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT7`"]
pub struct DOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DOUT8`"]
pub type DOUT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT8`"]
pub struct DOUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT8_W<'a> {
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
#[doc = "Reader of field `DOUT9`"]
pub type DOUT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT9`"]
pub struct DOUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT9_W<'a> {
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
#[doc = "Reader of field `DOUT10`"]
pub type DOUT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT10`"]
pub struct DOUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT10_W<'a> {
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
#[doc = "Reader of field `DOUT11`"]
pub type DOUT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT11`"]
pub struct DOUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT11_W<'a> {
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
#[doc = "Reader of field `DOUT12`"]
pub type DOUT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT12`"]
pub struct DOUT12_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT12_W<'a> {
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
#[doc = "Reader of field `DOUT13`"]
pub type DOUT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT13`"]
pub struct DOUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT13_W<'a> {
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
#[doc = "Reader of field `DOUT14`"]
pub type DOUT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT14`"]
pub struct DOUT14_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT14_W<'a> {
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
#[doc = "Reader of field `DOUT15`"]
pub type DOUT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT15`"]
pub struct DOUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    pub fn dout6(&self) -> DOUT6_R {
        DOUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    pub fn dout12(&self) -> DOUT12_R {
        DOUT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    pub fn dout0(&mut self) -> DOUT0_W {
        DOUT0_W { w: self }
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    pub fn dout1(&mut self) -> DOUT1_W {
        DOUT1_W { w: self }
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    pub fn dout2(&mut self) -> DOUT2_W {
        DOUT2_W { w: self }
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    pub fn dout3(&mut self) -> DOUT3_W {
        DOUT3_W { w: self }
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    pub fn dout4(&mut self) -> DOUT4_W {
        DOUT4_W { w: self }
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT5_W {
        DOUT5_W { w: self }
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    pub fn dout6(&mut self) -> DOUT6_W {
        DOUT6_W { w: self }
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    pub fn dout7(&mut self) -> DOUT7_W {
        DOUT7_W { w: self }
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    pub fn dout8(&mut self) -> DOUT8_W {
        DOUT8_W { w: self }
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    pub fn dout9(&mut self) -> DOUT9_W {
        DOUT9_W { w: self }
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    pub fn dout10(&mut self) -> DOUT10_W {
        DOUT10_W { w: self }
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    pub fn dout11(&mut self) -> DOUT11_W {
        DOUT11_W { w: self }
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    pub fn dout12(&mut self) -> DOUT12_W {
        DOUT12_W { w: self }
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    pub fn dout13(&mut self) -> DOUT13_W {
        DOUT13_W { w: self }
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    pub fn dout14(&mut self) -> DOUT14_W {
        DOUT14_W { w: self }
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    pub fn dout15(&mut self) -> DOUT15_W {
        DOUT15_W { w: self }
    }
}
