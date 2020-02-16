#[doc = "Reader of register PWRCU_BAKREG9"]
pub type R = crate::R<u32, super::PWRCU_BAKREG9>;
#[doc = "Writer for register PWRCU_BAKREG9"]
pub type W = crate::W<u32, super::PWRCU_BAKREG9>;
#[doc = "Register PWRCU_BAKREG9 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_BAKREG9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAKREG`"]
pub type BAKREG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BAKREG`"]
pub struct BAKREG_W<'a> {
    w: &'a mut W,
}
impl<'a> BAKREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    pub fn bakreg(&self) -> BAKREG_R {
        BAKREG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    pub fn bakreg(&mut self) -> BAKREG_W {
        BAKREG_W { w: self }
    }
}
