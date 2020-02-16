#[doc = "Reader of register CH0CCR"]
pub type R = crate::R<u32, super::CH0CCR>;
#[doc = "Writer for register CH0CCR"]
pub type W = crate::W<u32, super::CH0CCR>;
#[doc = "Register CH0CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CCV`"]
pub type CH0CCV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH0CCV`"]
pub struct CH0CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH0CCV"]
    #[inline(always)]
    pub fn ch0ccv(&self) -> CH0CCV_R {
        CH0CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0CCV"]
    #[inline(always)]
    pub fn ch0ccv(&mut self) -> CH0CCV_W {
        CH0CCV_W { w: self }
    }
}
