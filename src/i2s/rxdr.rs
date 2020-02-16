#[doc = "Reader of register RXDR"]
pub type R = crate::R<u32, super::RXDR>;
#[doc = "Writer for register RXDR"]
pub type W = crate::W<u32, super::RXDR>;
#[doc = "Register RXDR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDR`"]
pub type RXDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXDR`"]
pub struct RXDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RXDR_W {
        RXDR_W { w: self }
    }
}
