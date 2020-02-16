#[doc = "Reader of register LOCKR"]
pub type R = crate::R<u32, super::LOCKR>;
#[doc = "Writer for register LOCKR"]
pub type W = crate::W<u32, super::LOCKR>;
#[doc = "Register LOCKR `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK0`"]
pub type LOCK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK0`"]
pub struct LOCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK0_W<'a> {
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
#[doc = "Reader of field `LOCK1`"]
pub type LOCK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK1`"]
pub struct LOCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK1_W<'a> {
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
#[doc = "Reader of field `LOCK2`"]
pub type LOCK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK2`"]
pub struct LOCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK2_W<'a> {
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
#[doc = "Reader of field `LOCK3`"]
pub type LOCK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK3`"]
pub struct LOCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK3_W<'a> {
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
#[doc = "Reader of field `LOCK4`"]
pub type LOCK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK4`"]
pub struct LOCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK4_W<'a> {
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
#[doc = "Reader of field `LOCK5`"]
pub type LOCK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK5`"]
pub struct LOCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK5_W<'a> {
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
#[doc = "Reader of field `LOCK6`"]
pub type LOCK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK6`"]
pub struct LOCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK6_W<'a> {
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
#[doc = "Reader of field `LOCK7`"]
pub type LOCK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK7`"]
pub struct LOCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK7_W<'a> {
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
#[doc = "Reader of field `LOCK8`"]
pub type LOCK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK8`"]
pub struct LOCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK8_W<'a> {
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
#[doc = "Reader of field `LOCK9`"]
pub type LOCK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK9`"]
pub struct LOCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK9_W<'a> {
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
#[doc = "Reader of field `LOCK10`"]
pub type LOCK10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK10`"]
pub struct LOCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK10_W<'a> {
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
#[doc = "Reader of field `LOCK11`"]
pub type LOCK11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK11`"]
pub struct LOCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK11_W<'a> {
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
#[doc = "Reader of field `LOCK12`"]
pub type LOCK12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK12`"]
pub struct LOCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK12_W<'a> {
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
#[doc = "Reader of field `LOCK13`"]
pub type LOCK13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK13`"]
pub struct LOCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK13_W<'a> {
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
#[doc = "Reader of field `LOCK14`"]
pub type LOCK14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK14`"]
pub struct LOCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK14_W<'a> {
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
#[doc = "Reader of field `LOCK15`"]
pub type LOCK15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK15`"]
pub struct LOCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK15_W<'a> {
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
#[doc = "Reader of field `LKEY`"]
pub type LKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LKEY`"]
pub struct LKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> LKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK0_W {
        LOCK0_W { w: self }
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK1_W {
        LOCK1_W { w: self }
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK2_W {
        LOCK2_W { w: self }
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK3_W {
        LOCK3_W { w: self }
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK4_W {
        LOCK4_W { w: self }
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK5_W {
        LOCK5_W { w: self }
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK6_W {
        LOCK6_W { w: self }
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK7_W {
        LOCK7_W { w: self }
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&mut self) -> LOCK8_W {
        LOCK8_W { w: self }
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&mut self) -> LOCK9_W {
        LOCK9_W { w: self }
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&mut self) -> LOCK10_W {
        LOCK10_W { w: self }
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&mut self) -> LOCK11_W {
        LOCK11_W { w: self }
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&mut self) -> LOCK12_W {
        LOCK12_W { w: self }
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&mut self) -> LOCK13_W {
        LOCK13_W { w: self }
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&mut self) -> LOCK14_W {
        LOCK14_W { w: self }
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&mut self) -> LOCK15_W {
        LOCK15_W { w: self }
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W {
        LKEY_W { w: self }
    }
}
