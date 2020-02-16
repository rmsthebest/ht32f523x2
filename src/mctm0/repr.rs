#[doc = "Reader of register REPR"]
pub type R = crate::R<u32, super::REPR>;
#[doc = "Writer for register REPR"]
pub type W = crate::W<u32, super::REPR>;
#[doc = "Register REPR `reset()`'s with value 0"]
impl crate::ResetValue for super::REPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REPV`"]
pub type REPV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REPV`"]
pub struct REPV_W<'a> {
    w: &'a mut W,
}
impl<'a> REPV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    pub fn repv(&self) -> REPV_R {
        REPV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    pub fn repv(&mut self) -> REPV_W {
        REPV_W { w: self }
    }
}
