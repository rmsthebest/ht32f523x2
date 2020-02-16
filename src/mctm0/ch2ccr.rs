#[doc = "Reader of register CH2CCR"]
pub type R = crate::R<u32, super::CH2CCR>;
#[doc = "Writer for register CH2CCR"]
pub type W = crate::W<u32, super::CH2CCR>;
#[doc = "Register CH2CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2CCV`"]
pub type CH2CCV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH2CCV`"]
pub struct CH2CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&self) -> CH2CCV_R {
        CH2CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&mut self) -> CH2CCV_W {
        CH2CCV_W { w: self }
    }
}
