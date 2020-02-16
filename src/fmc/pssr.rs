#[doc = "Reader of register PSSR"]
pub type R = crate::R<u32, super::PSSR>;
#[doc = "Writer for register PSSR"]
pub type W = crate::W<u32, super::PSSR>;
#[doc = "Register PSSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSSB`"]
pub type PSSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PSSB`"]
pub struct PSSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    pub fn pssb(&self) -> PSSB_R {
        PSSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    pub fn pssb(&mut self) -> PSSB_W {
        PSSB_W { w: self }
    }
}
