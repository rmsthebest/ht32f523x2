#[doc = "Reader of register WCR"]
pub type R = crate::R<u32, super::WCR>;
#[doc = "Writer for register WCR"]
pub type W = crate::W<u32, super::WCR>;
#[doc = "Register WCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADWLE`"]
pub type ADWLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADWLE`"]
pub struct ADWLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADWLE_W<'a> {
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
#[doc = "Reader of field `ADWUE`"]
pub type ADWUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADWUE`"]
pub struct ADWUE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADWUE_W<'a> {
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
#[doc = "Reader of field `ADWALL`"]
pub type ADWALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADWALL`"]
pub struct ADWALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADWALL_W<'a> {
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
#[doc = "Reader of field `ADWCH`"]
pub type ADWCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADWCH`"]
pub struct ADWCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADWCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADLCH`"]
pub type ADLCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADLCH`"]
pub struct ADLCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADUCH`"]
pub type ADUCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADUCH`"]
pub struct ADUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADUCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    pub fn adwle(&self) -> ADWLE_R {
        ADWLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    pub fn adwue(&self) -> ADWUE_R {
        ADWUE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    pub fn adwall(&self) -> ADWALL_R {
        ADWALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    pub fn adwch(&self) -> ADWCH_R {
        ADWCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    pub fn adlch(&self) -> ADLCH_R {
        ADLCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    pub fn aduch(&self) -> ADUCH_R {
        ADUCH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADWLE"]
    #[inline(always)]
    pub fn adwle(&mut self) -> ADWLE_W {
        ADWLE_W { w: self }
    }
    #[doc = "Bit 1 - ADWUE"]
    #[inline(always)]
    pub fn adwue(&mut self) -> ADWUE_W {
        ADWUE_W { w: self }
    }
    #[doc = "Bit 2 - ADWALL"]
    #[inline(always)]
    pub fn adwall(&mut self) -> ADWALL_W {
        ADWALL_W { w: self }
    }
    #[doc = "Bits 8:11 - ADWCH"]
    #[inline(always)]
    pub fn adwch(&mut self) -> ADWCH_W {
        ADWCH_W { w: self }
    }
    #[doc = "Bits 16:19 - ADLCH"]
    #[inline(always)]
    pub fn adlch(&mut self) -> ADLCH_W {
        ADLCH_W { w: self }
    }
    #[doc = "Bits 24:27 - ADUCH"]
    #[inline(always)]
    pub fn aduch(&mut self) -> ADUCH_W {
        ADUCH_W { w: self }
    }
}
