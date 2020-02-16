#[doc = "Reader of register FSR"]
pub type R = crate::R<u32, super::FSR>;
#[doc = "Writer for register FSR"]
pub type W = crate::W<u32, super::FSR>;
#[doc = "Register FSR `reset()`'s with value 0"]
impl crate::ResetValue for super::FSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    pub fn txfs(&mut self) -> TXFS_W {
        TXFS_W { w: self }
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&mut self) -> RXFS_W {
        RXFS_W { w: self }
    }
}
