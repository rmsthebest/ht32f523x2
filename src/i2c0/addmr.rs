#[doc = "Reader of register ADDMR"]
pub type R = crate::R<u32, super::ADDMR>;
#[doc = "Writer for register ADDMR"]
pub type W = crate::W<u32, super::ADDMR>;
#[doc = "Register ADDMR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDMR`"]
pub type ADDMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDMR`"]
pub struct ADDMR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    pub fn addmr(&self) -> ADDMR_R {
        ADDMR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    pub fn addmr(&mut self) -> ADDMR_W {
        ADDMR_W { w: self }
    }
}
