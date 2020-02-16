#[doc = "Reader of register DEVAR"]
pub type R = crate::R<u32, super::DEVAR>;
#[doc = "Writer for register DEVAR"]
pub type W = crate::W<u32, super::DEVAR>;
#[doc = "Register DEVAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVA`"]
pub type DEVA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVA`"]
pub struct DEVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    pub fn deva(&self) -> DEVA_R {
        DEVA_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    pub fn deva(&mut self) -> DEVA_W {
        DEVA_W { w: self }
    }
}
