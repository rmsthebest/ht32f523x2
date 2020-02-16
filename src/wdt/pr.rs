#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Writer for register PR"]
pub type W = crate::W<u32, super::PR>;
#[doc = "Register PR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROTECT`"]
pub type PROTECT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROTECT`"]
pub struct PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PROTECT"]
    #[inline(always)]
    pub fn protect(&mut self) -> PROTECT_W {
        PROTECT_W { w: self }
    }
}
