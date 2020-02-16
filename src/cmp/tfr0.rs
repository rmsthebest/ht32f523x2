#[doc = "Reader of register TFR0"]
pub type R = crate::R<u32, super::TFR0>;
#[doc = "Writer for register TFR0"]
pub type W = crate::W<u32, super::TFR0>;
#[doc = "Register TFR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TFR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPFF`"]
pub type CMPFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPFF`"]
pub struct CMPFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFF_W<'a> {
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
#[doc = "Reader of field `CMPRF`"]
pub type CMPRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPRF`"]
pub struct CMPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPRF_W<'a> {
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
#[doc = "Reader of field `CMPFDEN`"]
pub type CMPFDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPFDEN`"]
pub struct CMPFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFDEN_W<'a> {
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
#[doc = "Reader of field `CMPRDEN`"]
pub type CMPRDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPRDEN`"]
pub struct CMPRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPRDEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    pub fn cmpff(&self) -> CMPFF_R {
        CMPFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    pub fn cmprf(&self) -> CMPRF_R {
        CMPRF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    pub fn cmpfden(&self) -> CMPFDEN_R {
        CMPFDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CMPRDEN"]
    #[inline(always)]
    pub fn cmprden(&self) -> CMPRDEN_R {
        CMPRDEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    pub fn cmpff(&mut self) -> CMPFF_W {
        CMPFF_W { w: self }
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    pub fn cmprf(&mut self) -> CMPRF_W {
        CMPRF_W { w: self }
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    pub fn cmpfden(&mut self) -> CMPFDEN_W {
        CMPFDEN_W { w: self }
    }
    #[doc = "Bit 9 - CMPRDEN"]
    #[inline(always)]
    pub fn cmprden(&mut self) -> CMPRDEN_W {
        CMPRDEN_W { w: self }
    }
}
