#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Writer for register PR"]
pub type W = crate::W<u32, super::PR>;
#[doc = "Register PR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSPOL`"]
pub type CSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPOL`"]
pub struct CSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPOL_W<'a> {
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
#[doc = "Reader of field `OEPOL`"]
pub type OEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEPOL`"]
pub struct OEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPOL_W<'a> {
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
#[doc = "Reader of field `WEPOL`"]
pub type WEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WEPOL`"]
pub struct WEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WEPOL_W<'a> {
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
#[doc = "Reader of field `ALEPOL`"]
pub type ALEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALEPOL`"]
pub struct ALEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEPOL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    pub fn cspol(&mut self) -> CSPOL_W {
        CSPOL_W { w: self }
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    pub fn oepol(&mut self) -> OEPOL_W {
        OEPOL_W { w: self }
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    pub fn wepol(&mut self) -> WEPOL_W {
        WEPOL_W { w: self }
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    pub fn alepol(&mut self) -> ALEPOL_W {
        ALEPOL_W { w: self }
    }
}
