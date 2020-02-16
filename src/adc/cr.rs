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
#[doc = "Reader of field `ADMODE`"]
pub type ADMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADMODE`"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADCRST`"]
pub type ADCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCRST`"]
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
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
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
#[doc = "Reader of field `ADSEQL`"]
pub type ADSEQL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQL`"]
pub struct ADSEQL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSUBL`"]
pub type ADSUBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSUBL`"]
pub struct ADSUBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSUBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 6 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&self) -> ADSEQL_R {
        ADSEQL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&self) -> ADSUBL_R {
        ADSUBL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bit 6 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    #[doc = "Bit 7 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bits 8:10 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&mut self) -> ADSEQL_W {
        ADSEQL_W { w: self }
    }
    #[doc = "Bits 16:18 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&mut self) -> ADSUBL_W {
        ADSUBL_W { w: self }
    }
}
