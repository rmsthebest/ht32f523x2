#[doc = "Reader of register RXB"]
pub type R = crate::R<u32, super::RXB>;
#[doc = "Writer for register RXB"]
pub type W = crate::W<u32, super::RXB>;
#[doc = "Register RXB `reset()`'s with value 0"]
impl crate::ResetValue for super::RXB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RB`"]
pub type RB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RB`"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
}
