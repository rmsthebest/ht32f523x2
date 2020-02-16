#[doc = "Reader of register USART_USRTPR"]
pub type R = crate::R<u32, super::USART_USRTPR>;
#[doc = "Writer for register USART_USRTPR"]
pub type W = crate::W<u32, super::USART_USRTPR>;
#[doc = "Register USART_USRTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXTOC`"]
pub type RXTOC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTOC`"]
pub struct RXTOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RXTOEN`"]
pub type RXTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTOEN`"]
pub struct RXTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOEN_W<'a> {
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
#[doc = "Reader of field `TG`"]
pub type TG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TG`"]
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    pub fn rxtoc(&self) -> RXTOC_R {
        RXTOC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    pub fn rxtoen(&self) -> RXTOEN_R {
        RXTOEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    pub fn rxtoc(&mut self) -> RXTOC_W {
        RXTOC_W { w: self }
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    pub fn rxtoen(&mut self) -> RXTOEN_W {
        RXTOEN_W { w: self }
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
}
