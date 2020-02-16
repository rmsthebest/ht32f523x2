#[doc = "Reader of register PSCR"]
pub type R = crate::R<u32, super::PSCR>;
#[doc = "Writer for register PSCR"]
pub type W = crate::W<u32, super::PSCR>;
#[doc = "Register PSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSCV`"]
pub type PSCV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSCV`"]
pub struct PSCV_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    pub fn pscv(&self) -> PSCV_R {
        PSCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSCV"]
    #[inline(always)]
    pub fn pscv(&mut self) -> PSCV_W {
        PSCV_W { w: self }
    }
}
