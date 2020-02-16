#[doc = "Reader of register PWRCU_LVDCSR"]
pub type R = crate::R<u32, super::PWRCU_LVDCSR>;
#[doc = "Writer for register PWRCU_LVDCSR"]
pub type W = crate::W<u32, super::PWRCU_LVDCSR>;
#[doc = "Register PWRCU_LVDCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_LVDCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BODEN`"]
pub type BODEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODEN`"]
pub struct BODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODEN_W<'a> {
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
#[doc = "Reader of field `BODRIS`"]
pub type BODRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODRIS`"]
pub struct BODRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRIS_W<'a> {
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
#[doc = "Reader of field `BODF`"]
pub type BODF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODF`"]
pub struct BODF_W<'a> {
    w: &'a mut W,
}
impl<'a> BODF_W<'a> {
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
#[doc = "Reader of field `LVDEN`"]
pub type LVDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDEN`"]
pub struct LVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDEN_W<'a> {
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
#[doc = "Reader of field `LVDS01`"]
pub type LVDS01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVDS01`"]
pub struct LVDS01_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDS01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `LVDS2`"]
pub type LVDS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDS2`"]
pub struct LVDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LVDF`"]
pub type LVDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDF`"]
pub struct LVDF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `LVDIWEN`"]
pub type LVDIWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDIWEN`"]
pub struct LVDIWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDIWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LVDEWEN`"]
pub type LVDEWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDEWEN`"]
pub struct LVDEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDEWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    pub fn boden(&self) -> BODEN_R {
        BODEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    pub fn bodris(&self) -> BODRIS_R {
        BODRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    pub fn bodf(&self) -> BODF_R {
        BODF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    pub fn lvds01(&self) -> LVDS01_R {
        LVDS01_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    pub fn lvds2(&self) -> LVDS2_R {
        LVDS2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    pub fn lvdiwen(&self) -> LVDIWEN_R {
        LVDIWEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    pub fn lvdewen(&self) -> LVDEWEN_R {
        LVDEWEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    pub fn boden(&mut self) -> BODEN_W {
        BODEN_W { w: self }
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    pub fn bodris(&mut self) -> BODRIS_W {
        BODRIS_W { w: self }
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    pub fn bodf(&mut self) -> BODF_W {
        BODF_W { w: self }
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    pub fn lvden(&mut self) -> LVDEN_W {
        LVDEN_W { w: self }
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    pub fn lvds01(&mut self) -> LVDS01_W {
        LVDS01_W { w: self }
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    pub fn lvds2(&mut self) -> LVDS2_W {
        LVDS2_W { w: self }
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    pub fn lvdf(&mut self) -> LVDF_W {
        LVDF_W { w: self }
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    pub fn lvdiwen(&mut self) -> LVDIWEN_W {
        LVDIWEN_W { w: self }
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    pub fn lvdewen(&mut self) -> LVDEWEN_W {
        LVDEWEN_W { w: self }
    }
}
