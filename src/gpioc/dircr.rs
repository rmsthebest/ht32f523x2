#[doc = "Reader of register DIRCR"]
pub type R = crate::R<u32, super::DIRCR>;
#[doc = "Writer for register DIRCR"]
pub type W = crate::W<u32, super::DIRCR>;
#[doc = "Register DIRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIR0`"]
pub type DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR0`"]
pub struct DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR0_W<'a> {
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
#[doc = "Reader of field `DIR1`"]
pub type DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR1`"]
pub struct DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR1_W<'a> {
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
#[doc = "Reader of field `DIR2`"]
pub type DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR2`"]
pub struct DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR2_W<'a> {
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
#[doc = "Reader of field `DIR3`"]
pub type DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR3`"]
pub struct DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR3_W<'a> {
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
#[doc = "Reader of field `DIR4`"]
pub type DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR4`"]
pub struct DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR4_W<'a> {
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
#[doc = "Reader of field `DIR5`"]
pub type DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR5`"]
pub struct DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR5_W<'a> {
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
#[doc = "Reader of field `DIR6`"]
pub type DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR6`"]
pub struct DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR6_W<'a> {
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
#[doc = "Reader of field `DIR7`"]
pub type DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR7`"]
pub struct DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR7_W<'a> {
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
#[doc = "Reader of field `DIR8`"]
pub type DIR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR8`"]
pub struct DIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR8_W<'a> {
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
#[doc = "Reader of field `DIR9`"]
pub type DIR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR9`"]
pub struct DIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR9_W<'a> {
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
#[doc = "Reader of field `DIR10`"]
pub type DIR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR10`"]
pub struct DIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR10_W<'a> {
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
#[doc = "Reader of field `DIR11`"]
pub type DIR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR11`"]
pub struct DIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR11_W<'a> {
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
#[doc = "Reader of field `DIR12`"]
pub type DIR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR12`"]
pub struct DIR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR12_W<'a> {
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
#[doc = "Reader of field `DIR13`"]
pub type DIR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR13`"]
pub struct DIR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR13_W<'a> {
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
#[doc = "Reader of field `DIR14`"]
pub type DIR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR14`"]
pub struct DIR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR14_W<'a> {
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
#[doc = "Reader of field `DIR15`"]
pub type DIR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR15`"]
pub struct DIR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR15_W<'a> {
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
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    pub fn dir6(&self) -> DIR6_R {
        DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    pub fn dir7(&self) -> DIR7_R {
        DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    pub fn dir8(&self) -> DIR8_R {
        DIR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    pub fn dir9(&self) -> DIR9_R {
        DIR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    pub fn dir10(&self) -> DIR10_R {
        DIR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    pub fn dir11(&self) -> DIR11_R {
        DIR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    pub fn dir12(&self) -> DIR12_R {
        DIR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    pub fn dir13(&self) -> DIR13_R {
        DIR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    pub fn dir14(&self) -> DIR14_R {
        DIR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    pub fn dir15(&self) -> DIR15_R {
        DIR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    pub fn dir0(&mut self) -> DIR0_W {
        DIR0_W { w: self }
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    pub fn dir1(&mut self) -> DIR1_W {
        DIR1_W { w: self }
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    pub fn dir2(&mut self) -> DIR2_W {
        DIR2_W { w: self }
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    pub fn dir3(&mut self) -> DIR3_W {
        DIR3_W { w: self }
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    pub fn dir4(&mut self) -> DIR4_W {
        DIR4_W { w: self }
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    pub fn dir5(&mut self) -> DIR5_W {
        DIR5_W { w: self }
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    pub fn dir6(&mut self) -> DIR6_W {
        DIR6_W { w: self }
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    pub fn dir7(&mut self) -> DIR7_W {
        DIR7_W { w: self }
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    pub fn dir8(&mut self) -> DIR8_W {
        DIR8_W { w: self }
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    pub fn dir9(&mut self) -> DIR9_W {
        DIR9_W { w: self }
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    pub fn dir10(&mut self) -> DIR10_W {
        DIR10_W { w: self }
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    pub fn dir11(&mut self) -> DIR11_W {
        DIR11_W { w: self }
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    pub fn dir12(&mut self) -> DIR12_W {
        DIR12_W { w: self }
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    pub fn dir13(&mut self) -> DIR13_W {
        DIR13_W { w: self }
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    pub fn dir14(&mut self) -> DIR14_W {
        DIR14_W { w: self }
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    pub fn dir15(&mut self) -> DIR15_W {
        DIR15_W { w: self }
    }
}
