#[doc = "Reader of register RR"]
pub type R = crate::R<u32, super::RR>;
#[doc = "Writer for register RR"]
pub type W = crate::W<u32, super::RR>;
#[doc = "Register RR `reset()`'s with value 0"]
impl crate::ResetValue for super::RR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RST0`"]
pub type RST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST0`"]
pub struct RST0_W<'a> {
    w: &'a mut W,
}
impl<'a> RST0_W<'a> {
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
#[doc = "Reader of field `RST1`"]
pub type RST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST1`"]
pub struct RST1_W<'a> {
    w: &'a mut W,
}
impl<'a> RST1_W<'a> {
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
#[doc = "Reader of field `RST2`"]
pub type RST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST2`"]
pub struct RST2_W<'a> {
    w: &'a mut W,
}
impl<'a> RST2_W<'a> {
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
#[doc = "Reader of field `RST3`"]
pub type RST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST3`"]
pub struct RST3_W<'a> {
    w: &'a mut W,
}
impl<'a> RST3_W<'a> {
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
#[doc = "Reader of field `RST4`"]
pub type RST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST4`"]
pub struct RST4_W<'a> {
    w: &'a mut W,
}
impl<'a> RST4_W<'a> {
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
#[doc = "Reader of field `RST5`"]
pub type RST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST5`"]
pub struct RST5_W<'a> {
    w: &'a mut W,
}
impl<'a> RST5_W<'a> {
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
#[doc = "Reader of field `RST6`"]
pub type RST6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST6`"]
pub struct RST6_W<'a> {
    w: &'a mut W,
}
impl<'a> RST6_W<'a> {
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
#[doc = "Reader of field `RST7`"]
pub type RST7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST7`"]
pub struct RST7_W<'a> {
    w: &'a mut W,
}
impl<'a> RST7_W<'a> {
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
#[doc = "Reader of field `RST8`"]
pub type RST8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST8`"]
pub struct RST8_W<'a> {
    w: &'a mut W,
}
impl<'a> RST8_W<'a> {
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
#[doc = "Reader of field `RST9`"]
pub type RST9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST9`"]
pub struct RST9_W<'a> {
    w: &'a mut W,
}
impl<'a> RST9_W<'a> {
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
#[doc = "Reader of field `RST10`"]
pub type RST10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST10`"]
pub struct RST10_W<'a> {
    w: &'a mut W,
}
impl<'a> RST10_W<'a> {
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
#[doc = "Reader of field `RST11`"]
pub type RST11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST11`"]
pub struct RST11_W<'a> {
    w: &'a mut W,
}
impl<'a> RST11_W<'a> {
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
#[doc = "Reader of field `RST12`"]
pub type RST12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST12`"]
pub struct RST12_W<'a> {
    w: &'a mut W,
}
impl<'a> RST12_W<'a> {
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
#[doc = "Reader of field `RST13`"]
pub type RST13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST13`"]
pub struct RST13_W<'a> {
    w: &'a mut W,
}
impl<'a> RST13_W<'a> {
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
#[doc = "Reader of field `RST14`"]
pub type RST14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST14`"]
pub struct RST14_W<'a> {
    w: &'a mut W,
}
impl<'a> RST14_W<'a> {
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
#[doc = "Reader of field `RST15`"]
pub type RST15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST15`"]
pub struct RST15_W<'a> {
    w: &'a mut W,
}
impl<'a> RST15_W<'a> {
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
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> RST4_R {
        RST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> RST5_R {
        RST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    pub fn rst6(&self) -> RST6_R {
        RST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    pub fn rst7(&self) -> RST7_R {
        RST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    pub fn rst8(&self) -> RST8_R {
        RST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    pub fn rst9(&self) -> RST9_R {
        RST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    pub fn rst10(&self) -> RST10_R {
        RST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    pub fn rst11(&self) -> RST11_R {
        RST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    pub fn rst12(&self) -> RST12_R {
        RST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    pub fn rst13(&self) -> RST13_R {
        RST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    pub fn rst14(&self) -> RST14_R {
        RST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    pub fn rst15(&self) -> RST15_R {
        RST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&mut self) -> RST0_W {
        RST0_W { w: self }
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&mut self) -> RST1_W {
        RST1_W { w: self }
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&mut self) -> RST2_W {
        RST2_W { w: self }
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&mut self) -> RST3_W {
        RST3_W { w: self }
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    pub fn rst4(&mut self) -> RST4_W {
        RST4_W { w: self }
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    pub fn rst5(&mut self) -> RST5_W {
        RST5_W { w: self }
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    pub fn rst6(&mut self) -> RST6_W {
        RST6_W { w: self }
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    pub fn rst7(&mut self) -> RST7_W {
        RST7_W { w: self }
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    pub fn rst8(&mut self) -> RST8_W {
        RST8_W { w: self }
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    pub fn rst9(&mut self) -> RST9_W {
        RST9_W { w: self }
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    pub fn rst10(&mut self) -> RST10_W {
        RST10_W { w: self }
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    pub fn rst11(&mut self) -> RST11_W {
        RST11_W { w: self }
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    pub fn rst12(&mut self) -> RST12_W {
        RST12_W { w: self }
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    pub fn rst13(&mut self) -> RST13_W {
        RST13_W { w: self }
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    pub fn rst14(&mut self) -> RST14_W {
        RST14_W { w: self }
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    pub fn rst15(&mut self) -> RST15_W {
        RST15_W { w: self }
    }
}
