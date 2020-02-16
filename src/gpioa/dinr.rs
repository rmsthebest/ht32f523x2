#[doc = "Reader of register DINR"]
pub type R = crate::R<u32, super::DINR>;
#[doc = "Writer for register DINR"]
pub type W = crate::W<u32, super::DINR>;
#[doc = "Register DINR `reset()`'s with value 0"]
impl crate::ResetValue for super::DINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIN0`"]
pub type DIN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN0`"]
pub struct DIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN0_W<'a> {
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
#[doc = "Reader of field `DIN1`"]
pub type DIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN1`"]
pub struct DIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN1_W<'a> {
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
#[doc = "Reader of field `DIN2`"]
pub type DIN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN2`"]
pub struct DIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN2_W<'a> {
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
#[doc = "Reader of field `DIN3`"]
pub type DIN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN3`"]
pub struct DIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN3_W<'a> {
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
#[doc = "Reader of field `DIN4`"]
pub type DIN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN4`"]
pub struct DIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN4_W<'a> {
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
#[doc = "Reader of field `DIN5`"]
pub type DIN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN5`"]
pub struct DIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN5_W<'a> {
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
#[doc = "Reader of field `DIN6`"]
pub type DIN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN6`"]
pub struct DIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN6_W<'a> {
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
#[doc = "Reader of field `DIN7`"]
pub type DIN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN7`"]
pub struct DIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN7_W<'a> {
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
#[doc = "Reader of field `DIN8`"]
pub type DIN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN8`"]
pub struct DIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN8_W<'a> {
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
#[doc = "Reader of field `DIN9`"]
pub type DIN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN9`"]
pub struct DIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN9_W<'a> {
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
#[doc = "Reader of field `DIN10`"]
pub type DIN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN10`"]
pub struct DIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN10_W<'a> {
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
#[doc = "Reader of field `DIN11`"]
pub type DIN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN11`"]
pub struct DIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN11_W<'a> {
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
#[doc = "Reader of field `DIN12`"]
pub type DIN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN12`"]
pub struct DIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN12_W<'a> {
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
#[doc = "Reader of field `DIN13`"]
pub type DIN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN13`"]
pub struct DIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN13_W<'a> {
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
#[doc = "Reader of field `DIN14`"]
pub type DIN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN14`"]
pub struct DIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN14_W<'a> {
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
#[doc = "Reader of field `DIN15`"]
pub type DIN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIN15`"]
pub struct DIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN15_W<'a> {
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
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    pub fn din5(&self) -> DIN5_R {
        DIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    pub fn din10(&self) -> DIN10_R {
        DIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    pub fn din15(&self) -> DIN15_R {
        DIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    pub fn din0(&mut self) -> DIN0_W {
        DIN0_W { w: self }
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    pub fn din1(&mut self) -> DIN1_W {
        DIN1_W { w: self }
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    pub fn din2(&mut self) -> DIN2_W {
        DIN2_W { w: self }
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    pub fn din3(&mut self) -> DIN3_W {
        DIN3_W { w: self }
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    pub fn din4(&mut self) -> DIN4_W {
        DIN4_W { w: self }
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    pub fn din5(&mut self) -> DIN5_W {
        DIN5_W { w: self }
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    pub fn din6(&mut self) -> DIN6_W {
        DIN6_W { w: self }
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    pub fn din7(&mut self) -> DIN7_W {
        DIN7_W { w: self }
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    pub fn din8(&mut self) -> DIN8_W {
        DIN8_W { w: self }
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    pub fn din9(&mut self) -> DIN9_W {
        DIN9_W { w: self }
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    pub fn din10(&mut self) -> DIN10_W {
        DIN10_W { w: self }
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    pub fn din11(&mut self) -> DIN11_W {
        DIN11_W { w: self }
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    pub fn din12(&mut self) -> DIN12_W {
        DIN12_W { w: self }
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    pub fn din13(&mut self) -> DIN13_W {
        DIN13_W { w: self }
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    pub fn din14(&mut self) -> DIN14_W {
        DIN14_W { w: self }
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    pub fn din15(&mut self) -> DIN15_W {
        DIN15_W { w: self }
    }
}
