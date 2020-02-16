#[doc = "Reader of register PWRCU_BAKTEST"]
pub type R = crate::R<u32, super::PWRCU_BAKTEST>;
#[doc = "Writer for register PWRCU_BAKTEST"]
pub type W = crate::W<u32, super::PWRCU_BAKTEST>;
#[doc = "Register PWRCU_BAKTEST `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_BAKTEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAKTEST`"]
pub type BAKTEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAKTEST`"]
pub struct BAKTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BAKTEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    pub fn baktest(&self) -> BAKTEST_R {
        BAKTEST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    pub fn baktest(&mut self) -> BAKTEST_W {
        BAKTEST_W { w: self }
    }
}
