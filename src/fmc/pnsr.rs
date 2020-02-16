#[doc = "Reader of register PNSR"]
pub type R = crate::R<u32, super::PNSR>;
#[doc = "Writer for register PNSR"]
pub type W = crate::W<u32, super::PNSR>;
#[doc = "Register PNSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PNSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PNSB`"]
pub type PNSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PNSB`"]
pub struct PNSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PNSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    pub fn pnsb(&self) -> PNSB_R {
        PNSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PNSB"]
    #[inline(always)]
    pub fn pnsb(&mut self) -> PNSB_W {
        PNSB_W { w: self }
    }
}
