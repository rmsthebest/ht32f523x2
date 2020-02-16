#[doc = "Reader of register RCNTR"]
pub type R = crate::R<u32, super::RCNTR>;
#[doc = "Writer for register RCNTR"]
pub type W = crate::W<u32, super::RCNTR>;
#[doc = "Register RCNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCNTR`"]
pub type RCNTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RCNTR`"]
pub struct RCNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RCNTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    pub fn rcntr(&self) -> RCNTR_R {
        RCNTR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - RCNTR"]
    #[inline(always)]
    pub fn rcntr(&mut self) -> RCNTR_W {
        RCNTR_W { w: self }
    }
}
