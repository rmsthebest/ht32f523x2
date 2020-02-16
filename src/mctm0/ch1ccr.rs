#[doc = "Reader of register CH1CCR"]
pub type R = crate::R<u32, super::CH1CCR>;
#[doc = "Writer for register CH1CCR"]
pub type W = crate::W<u32, super::CH1CCR>;
#[doc = "Register CH1CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1CCV`"]
pub type CH1CCV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1CCV`"]
pub struct CH1CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    pub fn ch1ccv(&self) -> CH1CCV_R {
        CH1CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    pub fn ch1ccv(&mut self) -> CH1CCV_W {
        CH1CCV_W { w: self }
    }
}
