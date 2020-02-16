#[doc = "Reader of register VALR1"]
pub type R = crate::R<u32, super::VALR1>;
#[doc = "Writer for register VALR1"]
pub type W = crate::W<u32, super::VALR1>;
#[doc = "Register VALR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VALR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CVRVAL`"]
pub type CVRVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CVRVAL`"]
pub struct CVRVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    pub fn cvrval(&self) -> CVRVAL_R {
        CVRVAL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    pub fn cvrval(&mut self) -> CVRVAL_W {
        CVRVAL_W { w: self }
    }
}
