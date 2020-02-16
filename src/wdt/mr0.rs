#[doc = "Reader of register MR0"]
pub type R = crate::R<u32, super::MR0>;
#[doc = "Writer for register MR0"]
pub type W = crate::W<u32, super::MR0>;
#[doc = "Register MR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTV`"]
pub type WDTV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDTV`"]
pub struct WDTV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `WDTFIEN`"]
pub type WDTFIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTFIEN`"]
pub struct WDTFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTFIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WDTRSTEN`"]
pub type WDTRSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRSTEN`"]
pub struct WDTRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `WDTSHLT`"]
pub type WDTSHLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDTSHLT`"]
pub struct WDTSHLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSHLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `WDTEN`"]
pub type WDTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTEN`"]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    pub fn wdtv(&self) -> WDTV_R {
        WDTV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    pub fn wdtfien(&self) -> WDTFIEN_R {
        WDTFIEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    pub fn wdtrsten(&self) -> WDTRSTEN_R {
        WDTRSTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    pub fn wdtshlt(&self) -> WDTSHLT_R {
        WDTSHLT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    pub fn wdtv(&mut self) -> WDTV_W {
        WDTV_W { w: self }
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    pub fn wdtfien(&mut self) -> WDTFIEN_W {
        WDTFIEN_W { w: self }
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    pub fn wdtrsten(&mut self) -> WDTRSTEN_W {
        WDTRSTEN_W { w: self }
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    pub fn wdtshlt(&mut self) -> WDTSHLT_W {
        WDTSHLT_W { w: self }
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
}
