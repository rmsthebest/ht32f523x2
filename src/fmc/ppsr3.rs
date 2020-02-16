#[doc = "Reader of register PPSR3"]
pub type R = crate::R<u32, super::PPSR3>;
#[doc = "Writer for register PPSR3"]
pub type W = crate::W<u32, super::PPSR3>;
#[doc = "Register PPSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PPSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSB`"]
pub type PPSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PPSB`"]
pub struct PPSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PPSB"]
    #[inline(always)]
    pub fn ppsb(&self) -> PPSB_R {
        PPSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSB"]
    #[inline(always)]
    pub fn ppsb(&mut self) -> PPSB_W {
        PPSB_W { w: self }
    }
}
