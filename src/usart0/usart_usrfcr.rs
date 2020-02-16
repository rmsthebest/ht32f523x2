#[doc = "Reader of register USART_USRFCR"]
pub type R = crate::R<u32, super::USART_USRFCR>;
#[doc = "Writer for register USART_USRFCR"]
pub type W = crate::W<u32, super::USART_USRFCR>;
#[doc = "Register USART_USRFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXR`"]
pub type TXR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXR`"]
pub struct TXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXR_W<'a> {
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
#[doc = "Reader of field `RXR`"]
pub type RXR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXR`"]
pub struct RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXR_W<'a> {
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
#[doc = "Reader of field `TXTL`"]
pub type TXTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTL`"]
pub struct TXTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RXTL`"]
pub type RXTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTL`"]
pub struct RXTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXFS`"]
pub type TXFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFS`"]
pub struct TXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXFS`"]
pub type RXFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFS`"]
pub struct RXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    pub fn txtl(&self) -> TXTL_R {
        TXTL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    pub fn rxtl(&self) -> RXTL_R {
        RXTL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    pub fn txr(&mut self) -> TXR_W {
        TXR_W { w: self }
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    pub fn rxr(&mut self) -> RXR_W {
        RXR_W { w: self }
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    pub fn txtl(&mut self) -> TXTL_W {
        TXTL_W { w: self }
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    pub fn rxtl(&mut self) -> RXTL_W {
        RXTL_W { w: self }
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    pub fn txfs(&mut self) -> TXFS_W {
        TXFS_W { w: self }
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&mut self) -> RXFS_W {
        RXFS_W { w: self }
    }
}
