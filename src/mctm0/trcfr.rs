#[doc = "Reader of register TRCFR"]
pub type R = crate::R<u32, super::TRCFR>;
#[doc = "Writer for register TRCFR"]
pub type W = crate::W<u32, super::TRCFR>;
#[doc = "Register TRCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRSEL`"]
pub type TRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRSEL`"]
pub struct TRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&mut self) -> TRSEL_W {
        TRSEL_W { w: self }
    }
}
