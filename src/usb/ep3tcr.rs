#[doc = "Reader of register EP3TCR"]
pub type R = crate::R<u32, super::EP3TCR>;
#[doc = "Writer for register EP3TCR"]
pub type W = crate::W<u32, super::EP3TCR>;
#[doc = "Register EP3TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP3TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCNT`"]
pub type TCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCNT`"]
pub struct TCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    pub fn tcnt(&self) -> TCNT_R {
        TCNT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    pub fn tcnt(&mut self) -> TCNT_W {
        TCNT_W { w: self }
    }
}
