#[doc = "Reader of register EP7CFGR"]
pub type R = crate::R<u32, super::EP7CFGR>;
#[doc = "Writer for register EP7CFGR"]
pub type W = crate::W<u32, super::EP7CFGR>;
#[doc = "Register EP7CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP7CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPBUFA`"]
pub type EPBUFA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EPBUFA`"]
pub struct EPBUFA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBUFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `EPLEN`"]
pub type EPLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EPLEN`"]
pub struct EPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `SDBS`"]
pub type SDBS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDBS`"]
pub struct SDBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `EPADR`"]
pub type EPADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPADR`"]
pub struct EPADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EPDIR`"]
pub type EPDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDIR`"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `EPTYPE`"]
pub type EPTYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPTYPE`"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `EPEN`"]
pub type EPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN`"]
pub struct EPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN_W<'a> {
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
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    pub fn epbufa(&self) -> EPBUFA_R {
        EPBUFA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - EPLEN"]
    #[inline(always)]
    pub fn eplen(&self) -> EPLEN_R {
        EPLEN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bit 23 - SDBS"]
    #[inline(always)]
    pub fn sdbs(&self) -> SDBS_R {
        SDBS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    pub fn epadr(&self) -> EPADR_R {
        EPADR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    pub fn epbufa(&mut self) -> EPBUFA_W {
        EPBUFA_W { w: self }
    }
    #[doc = "Bits 10:19 - EPLEN"]
    #[inline(always)]
    pub fn eplen(&mut self) -> EPLEN_W {
        EPLEN_W { w: self }
    }
    #[doc = "Bit 23 - SDBS"]
    #[inline(always)]
    pub fn sdbs(&mut self) -> SDBS_W {
        SDBS_W { w: self }
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    pub fn epadr(&mut self) -> EPADR_W {
        EPADR_W { w: self }
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    pub fn epen(&mut self) -> EPEN_W {
        EPEN_W { w: self }
    }
}
