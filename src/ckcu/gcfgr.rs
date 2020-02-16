#[doc = "Reader of register GCFGR"]
pub type R = crate::R<u32, super::GCFGR>;
#[doc = "Writer for register GCFGR"]
pub type W = crate::W<u32, super::GCFGR>;
#[doc = "Register GCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKOUTSRC`"]
pub type CKOUTSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKOUTSRC`"]
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
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
#[doc = "Reader of field `CKREFPRE`"]
pub type CKREFPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKREFPRE`"]
pub struct CKREFPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKREFPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `USBPRE`"]
pub type USBPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBPRE`"]
pub struct USBPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `LPMOD`"]
pub type LPMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMOD`"]
pub struct LPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    pub fn ckrefpre(&self) -> CKREFPRE_R {
        CKREFPRE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&self) -> LPMOD_R {
        LPMOD_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    pub fn ckrefpre(&mut self) -> CKREFPRE_W {
        CKREFPRE_W { w: self }
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    pub fn usbpre(&mut self) -> USBPRE_W {
        USBPRE_W { w: self }
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&mut self) -> LPMOD_W {
        LPMOD_W { w: self }
    }
}
