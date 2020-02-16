#[doc = "Reader of register CFGR11"]
pub type R = crate::R<u32, super::CFGR11>;
#[doc = "Writer for register CFGR11"]
pub type W = crate::W<u32, super::CFGR11>;
#[doc = "Register CFGR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBCNT`"]
pub type DBCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DBCNT`"]
pub struct DBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SRCTYPE`"]
pub type SRCTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCTYPE`"]
pub struct SRCTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `DBEN`"]
pub type DBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBEN`"]
pub struct DBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    pub fn dbcnt(&self) -> DBCNT_R {
        DBCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&self) -> SRCTYPE_R {
        SRCTYPE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    pub fn dbcnt(&mut self) -> DBCNT_W {
        DBCNT_W { w: self }
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&mut self) -> SRCTYPE_W {
        SRCTYPE_W { w: self }
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&mut self) -> DBEN_W {
        DBEN_W { w: self }
    }
}
