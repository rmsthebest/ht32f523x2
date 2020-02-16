#[doc = "Reader of register TXB"]
pub type R = crate::R<u32, super::TXB>;
#[doc = "Writer for register TXB"]
pub type W = crate::W<u32, super::TXB>;
#[doc = "Register TXB `reset()`'s with value 0"]
impl crate::ResetValue for super::TXB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TB`"]
pub type TB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TB`"]
pub struct TB_W<'a> {
    w: &'a mut W,
}
impl<'a> TB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&mut self) -> TB_W {
        TB_W { w: self }
    }
}
