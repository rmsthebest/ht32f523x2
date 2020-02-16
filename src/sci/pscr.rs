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
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
