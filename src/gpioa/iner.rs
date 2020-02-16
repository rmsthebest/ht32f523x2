#[doc = "Reader of register INER"]
pub type R = crate::R<u32, super::INER>;
#[doc = "Writer for register INER"]
pub type W = crate::W<u32, super::INER>;
#[doc = "Register INER `reset()`'s with value 0"]
impl crate::ResetValue for super::INER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INEN0`"]
pub type INEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN0`"]
pub struct INEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN0_W<'a> {
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
#[doc = "Reader of field `INEN1`"]
pub type INEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN1`"]
pub struct INEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN1_W<'a> {
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
#[doc = "Reader of field `INEN2`"]
pub type INEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN2`"]
pub struct INEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN2_W<'a> {
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
#[doc = "Reader of field `INEN3`"]
pub type INEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN3`"]
pub struct INEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN3_W<'a> {
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
#[doc = "Reader of field `INEN4`"]
pub type INEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN4`"]
pub struct INEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN4_W<'a> {
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
#[doc = "Reader of field `INEN5`"]
pub type INEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN5`"]
pub struct INEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN5_W<'a> {
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
#[doc = "Reader of field `INEN6`"]
pub type INEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN6`"]
pub struct INEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN6_W<'a> {
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
#[doc = "Reader of field `INEN7`"]
pub type INEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN7`"]
pub struct INEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN7_W<'a> {
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
#[doc = "Reader of field `INEN8`"]
pub type INEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN8`"]
pub struct INEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN8_W<'a> {
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
#[doc = "Reader of field `INEN9`"]
pub type INEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN9`"]
pub struct INEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN9_W<'a> {
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
#[doc = "Reader of field `INEN10`"]
pub type INEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN10`"]
pub struct INEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN10_W<'a> {
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
#[doc = "Reader of field `INEN11`"]
pub type INEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN11`"]
pub struct INEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN11_W<'a> {
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
#[doc = "Reader of field `INEN12`"]
pub type INEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN12`"]
pub struct INEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN12_W<'a> {
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
#[doc = "Reader of field `INEN13`"]
pub type INEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN13`"]
pub struct INEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN13_W<'a> {
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
#[doc = "Reader of field `INEN14`"]
pub type INEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN14`"]
pub struct INEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN14_W<'a> {
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
#[doc = "Reader of field `INEN15`"]
pub type INEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN15`"]
pub struct INEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN15_W<'a> {
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
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    pub fn inen0(&self) -> INEN0_R {
        INEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    pub fn inen1(&self) -> INEN1_R {
        INEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    pub fn inen2(&self) -> INEN2_R {
        INEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    pub fn inen3(&self) -> INEN3_R {
        INEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    pub fn inen4(&self) -> INEN4_R {
        INEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    pub fn inen5(&self) -> INEN5_R {
        INEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    pub fn inen6(&self) -> INEN6_R {
        INEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    pub fn inen7(&self) -> INEN7_R {
        INEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    pub fn inen8(&self) -> INEN8_R {
        INEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    pub fn inen9(&self) -> INEN9_R {
        INEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    pub fn inen10(&self) -> INEN10_R {
        INEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    pub fn inen11(&self) -> INEN11_R {
        INEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    pub fn inen12(&self) -> INEN12_R {
        INEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    pub fn inen13(&self) -> INEN13_R {
        INEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    pub fn inen14(&self) -> INEN14_R {
        INEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    pub fn inen15(&self) -> INEN15_R {
        INEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    pub fn inen0(&mut self) -> INEN0_W {
        INEN0_W { w: self }
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    pub fn inen1(&mut self) -> INEN1_W {
        INEN1_W { w: self }
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    pub fn inen2(&mut self) -> INEN2_W {
        INEN2_W { w: self }
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    pub fn inen3(&mut self) -> INEN3_W {
        INEN3_W { w: self }
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    pub fn inen4(&mut self) -> INEN4_W {
        INEN4_W { w: self }
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    pub fn inen5(&mut self) -> INEN5_W {
        INEN5_W { w: self }
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    pub fn inen6(&mut self) -> INEN6_W {
        INEN6_W { w: self }
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    pub fn inen7(&mut self) -> INEN7_W {
        INEN7_W { w: self }
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    pub fn inen8(&mut self) -> INEN8_W {
        INEN8_W { w: self }
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    pub fn inen9(&mut self) -> INEN9_W {
        INEN9_W { w: self }
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    pub fn inen10(&mut self) -> INEN10_W {
        INEN10_W { w: self }
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    pub fn inen11(&mut self) -> INEN11_W {
        INEN11_W { w: self }
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    pub fn inen12(&mut self) -> INEN12_W {
        INEN12_W { w: self }
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    pub fn inen13(&mut self) -> INEN13_W {
        INEN13_W { w: self }
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    pub fn inen14(&mut self) -> INEN14_W {
        INEN14_W { w: self }
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    pub fn inen15(&mut self) -> INEN15_W {
        INEN15_W { w: self }
    }
}
