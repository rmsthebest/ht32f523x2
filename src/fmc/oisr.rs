#[doc = "Reader of register OISR"]
pub type R = crate::R<u32, super::OISR>;
#[doc = "Writer for register OISR"]
pub type W = crate::W<u32, super::OISR>;
#[doc = "Register OISR `reset()`'s with value 0"]
impl crate::ResetValue for super::OISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ORFF`"]
pub type ORFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ORFF`"]
pub struct ORFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ORFF_W<'a> {
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
#[doc = "Reader of field `ITADF`"]
pub type ITADF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITADF`"]
pub struct ITADF_W<'a> {
    w: &'a mut W,
}
impl<'a> ITADF_W<'a> {
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
#[doc = "Reader of field `OBEF`"]
pub type OBEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBEF`"]
pub struct OBEF_W<'a> {
    w: &'a mut W,
}
impl<'a> OBEF_W<'a> {
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
#[doc = "Reader of field `IOCMF`"]
pub type IOCMF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOCMF`"]
pub struct IOCMF_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCMF_W<'a> {
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
#[doc = "Reader of field `OREF`"]
pub type OREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OREF`"]
pub struct OREF_W<'a> {
    w: &'a mut W,
}
impl<'a> OREF_W<'a> {
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
#[doc = "Reader of field `RORFF`"]
pub type RORFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RORFF`"]
pub struct RORFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RORFF_W<'a> {
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
#[doc = "Reader of field `PPEF`"]
pub type PPEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPEF`"]
pub struct PPEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PPEF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    pub fn orff(&self) -> ORFF_R {
        ORFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    pub fn itadf(&self) -> ITADF_R {
        ITADF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    pub fn obef(&self) -> OBEF_R {
        OBEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    pub fn iocmf(&self) -> IOCMF_R {
        IOCMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    pub fn oref(&self) -> OREF_R {
        OREF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    pub fn rorff(&self) -> RORFF_R {
        RORFF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    pub fn ppef(&self) -> PPEF_R {
        PPEF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    pub fn orff(&mut self) -> ORFF_W {
        ORFF_W { w: self }
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    pub fn itadf(&mut self) -> ITADF_W {
        ITADF_W { w: self }
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    pub fn obef(&mut self) -> OBEF_W {
        OBEF_W { w: self }
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    pub fn iocmf(&mut self) -> IOCMF_W {
        IOCMF_W { w: self }
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    pub fn oref(&mut self) -> OREF_W {
        OREF_W { w: self }
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    pub fn rorff(&mut self) -> RORFF_W {
        RORFF_W { w: self }
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    pub fn ppef(&mut self) -> PPEF_W {
        PPEF_W { w: self }
    }
}
