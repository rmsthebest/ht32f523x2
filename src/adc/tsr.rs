#[doc = "Reader of register TSR"]
pub type R = crate::R<u32, super::TSR>;
#[doc = "Writer for register TSR"]
pub type W = crate::W<u32, super::TSR>;
#[doc = "Register TSR `reset()`'s with value 0"]
impl crate::ResetValue for super::TSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSC`"]
pub type ADSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADSC`"]
pub struct ADSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSC_W<'a> {
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
#[doc = "Reader of field `ADEXTIS`"]
pub type ADEXTIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADEXTIS`"]
pub struct ADEXTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEXTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TMS`"]
pub type TMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMS`"]
pub struct TMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `BFTMS`"]
pub type BFTMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTMS`"]
pub struct BFTMS_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTMS_W<'a> {
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
#[doc = "Reader of field `CMPS`"]
pub type CMPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPS`"]
pub struct CMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPS_W<'a> {
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
#[doc = "Reader of field `TME`"]
pub type TME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TME`"]
pub struct TME_W<'a> {
    w: &'a mut W,
}
impl<'a> TME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> ADEXTIS_R {
        ADEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    pub fn tms(&self) -> TMS_R {
        TMS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    pub fn bftms(&self) -> BFTMS_R {
        BFTMS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    pub fn cmps(&self) -> CMPS_R {
        CMPS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&mut self) -> ADSC_W {
        ADSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&mut self) -> ADEXTIS_W {
        ADEXTIS_W { w: self }
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    pub fn tms(&mut self) -> TMS_W {
        TMS_W { w: self }
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    pub fn bftms(&mut self) -> BFTMS_W {
        BFTMS_W { w: self }
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    pub fn cmps(&mut self) -> CMPS_W {
        CMPS_W { w: self }
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W {
        TME_W { w: self }
    }
}
