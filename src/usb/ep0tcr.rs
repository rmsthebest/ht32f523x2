#[doc = "Reader of register EP0TCR"]
pub type R = crate::R<u32, super::EP0TCR>;
#[doc = "Writer for register EP0TCR"]
pub type W = crate::W<u32, super::EP0TCR>;
#[doc = "Register EP0TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP0TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXCNT`"]
pub type TXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCNT`"]
pub struct TXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RXCNT`"]
pub type RXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXCNT`"]
pub struct RXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    pub fn txcnt(&mut self) -> TXCNT_W {
        TXCNT_W { w: self }
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    pub fn rxcnt(&mut self) -> RXCNT_W {
        RXCNT_W { w: self }
    }
}
