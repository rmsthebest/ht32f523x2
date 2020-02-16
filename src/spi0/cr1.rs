#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DFL`"]
pub type DFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFL`"]
pub struct DFL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SELAP`"]
pub type SELAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELAP`"]
pub struct SELAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FIRSTBIT`"]
pub type FIRSTBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIRSTBIT`"]
pub struct FIRSTBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRSTBIT_W<'a> {
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
#[doc = "Reader of field `SELM`"]
pub type SELM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELM`"]
pub struct SELM_W<'a> {
    w: &'a mut W,
}
impl<'a> SELM_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    pub fn dfl(&self) -> DFL_R {
        DFL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    pub fn selap(&self) -> SELAP_R {
        SELAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    pub fn firstbit(&self) -> FIRSTBIT_R {
        FIRSTBIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    pub fn dfl(&mut self) -> DFL_W {
        DFL_W { w: self }
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    pub fn selap(&mut self) -> SELAP_W {
        SELAP_W { w: self }
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    pub fn firstbit(&mut self) -> FIRSTBIT_W {
        FIRSTBIT_W { w: self }
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    pub fn selm(&mut self) -> SELM_W {
        SELM_W { w: self }
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
