#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXFTLS`"]
pub type TXFTLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFTLS`"]
pub struct TXFTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RXFTLS`"]
pub type RXFTLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFTLS`"]
pub struct RXFTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXFRST`"]
pub type TXFRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFRST`"]
pub struct TXFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRST_W<'a> {
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
#[doc = "Reader of field `RXFRST`"]
pub type RXFRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFRST`"]
pub struct RXFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRST_W<'a> {
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
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&self) -> TXFTLS_R {
        TXFTLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&self) -> RXFTLS_R {
        RXFTLS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    pub fn txfrst(&self) -> TXFRST_R {
        TXFRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    pub fn rxfrst(&self) -> RXFRST_R {
        RXFRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&mut self) -> TXFTLS_W {
        TXFTLS_W { w: self }
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&mut self) -> RXFTLS_W {
        RXFTLS_W { w: self }
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    pub fn txfrst(&mut self) -> TXFRST_W {
        TXFRST_W { w: self }
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    pub fn rxfrst(&mut self) -> RXFRST_W {
        RXFRST_W { w: self }
    }
}
