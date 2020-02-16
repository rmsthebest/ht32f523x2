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
#[doc = "Reader of field `POLY`"]
pub type POLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLY`"]
pub struct POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DATBIRV`"]
pub type DATBIRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATBIRV`"]
pub struct DATBIRV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATBIRV_W<'a> {
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
#[doc = "Reader of field `DATBYRV`"]
pub type DATBYRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATBYRV`"]
pub struct DATBYRV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATBYRV_W<'a> {
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
#[doc = "Reader of field `DATCMPL`"]
pub type DATCMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATCMPL`"]
pub struct DATCMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCMPL_W<'a> {
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
#[doc = "Reader of field `SUMBIRV`"]
pub type SUMBIRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUMBIRV`"]
pub struct SUMBIRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUMBIRV_W<'a> {
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
#[doc = "Reader of field `SUMBYRV`"]
pub type SUMBYRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUMBYRV`"]
pub struct SUMBYRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUMBYRV_W<'a> {
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
#[doc = "Reader of field `SUMCMPL`"]
pub type SUMCMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUMCMPL`"]
pub struct SUMCMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUMCMPL_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    pub fn datbirv(&self) -> DATBIRV_R {
        DATBIRV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    pub fn datbyrv(&self) -> DATBYRV_R {
        DATBYRV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    pub fn datcmpl(&self) -> DATCMPL_R {
        DATCMPL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    pub fn sumbirv(&self) -> SUMBIRV_R {
        SUMBIRV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    pub fn sumbyrv(&self) -> SUMBYRV_R {
        SUMBYRV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    pub fn sumcmpl(&self) -> SUMCMPL_R {
        SUMCMPL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    pub fn poly(&mut self) -> POLY_W {
        POLY_W { w: self }
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    pub fn datbirv(&mut self) -> DATBIRV_W {
        DATBIRV_W { w: self }
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    pub fn datbyrv(&mut self) -> DATBYRV_W {
        DATBYRV_W { w: self }
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    pub fn datcmpl(&mut self) -> DATCMPL_W {
        DATCMPL_W { w: self }
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    pub fn sumbirv(&mut self) -> SUMBIRV_W {
        SUMBIRV_W { w: self }
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    pub fn sumbyrv(&mut self) -> SUMBYRV_W {
        SUMBYRV_W { w: self }
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    pub fn sumcmpl(&mut self) -> SUMCMPL_W {
        SUMCMPL_W { w: self }
    }
}
