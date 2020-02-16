#[doc = "Reader of register LPCR"]
pub type R = crate::R<u32, super::LPCR>;
#[doc = "Writer for register LPCR"]
pub type W = crate::W<u32, super::LPCR>;
#[doc = "Register LPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKISO`"]
pub type BKISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKISO`"]
pub struct BKISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BKISO_W<'a> {
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
#[doc = "Reader of field `USBSLEEP`"]
pub type USBSLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSLEEP`"]
pub struct USBSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSLEEP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    pub fn bkiso(&self) -> BKISO_R {
        BKISO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&self) -> USBSLEEP_R {
        USBSLEEP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    pub fn bkiso(&mut self) -> BKISO_W {
        BKISO_W { w: self }
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&mut self) -> USBSLEEP_W {
        USBSLEEP_W { w: self }
    }
}
