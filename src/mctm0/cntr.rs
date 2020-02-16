#[doc = "Reader of register CNTR"]
pub type R = crate::R<u32, super::CNTR>;
#[doc = "Writer for register CNTR"]
pub type W = crate::W<u32, super::CNTR>;
#[doc = "Register CNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTV`"]
pub type CNTV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNTV`"]
pub struct CNTV_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    pub fn cntv(&self) -> CNTV_R {
        CNTV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNTV"]
    #[inline(always)]
    pub fn cntv(&mut self) -> CNTV_W {
        CNTV_W { w: self }
    }
}
