#[doc = "Reader of register CH4SADR"]
pub type R = crate::R<u32, super::CH4SADR>;
#[doc = "Writer for register CH4SADR"]
pub type W = crate::W<u32, super::CH4SADR>;
#[doc = "Register CH4SADR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH4SADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADR`"]
pub type SADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SADR`"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
}
