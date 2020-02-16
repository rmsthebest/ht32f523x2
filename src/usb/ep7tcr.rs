#[doc = "Reader of register EP7TCR"]
pub type R = crate::R<u32, super::EP7TCR>;
#[doc = "Writer for register EP7TCR"]
pub type W = crate::W<u32, super::EP7TCR>;
#[doc = "Register EP7TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP7TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCNT0`"]
pub type TCNT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCNT0`"]
pub struct TCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCNT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `TCNT1`"]
pub type TCNT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCNT1`"]
pub struct TCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    pub fn tcnt0(&self) -> TCNT0_R {
        TCNT0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    pub fn tcnt1(&self) -> TCNT1_R {
        TCNT1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    pub fn tcnt0(&mut self) -> TCNT0_W {
        TCNT0_W { w: self }
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    pub fn tcnt1(&mut self) -> TCNT1_W {
        TCNT1_W { w: self }
    }
}
