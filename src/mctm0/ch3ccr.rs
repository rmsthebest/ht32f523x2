#[doc = "Reader of register CH3CCR"]
pub type R = crate::R<u32, super::CH3CCR>;
#[doc = "Writer for register CH3CCR"]
pub type W = crate::W<u32, super::CH3CCR>;
#[doc = "Register CH3CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3CCV`"]
pub type CH3CCV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH3CCV`"]
pub struct CH3CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&self) -> CH3CCV_R {
        CH3CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&mut self) -> CH3CCV_W {
        CH3CCV_W { w: self }
    }
}
