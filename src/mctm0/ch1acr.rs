#[doc = "Reader of register CH1ACR"]
pub type R = crate::R<u32, super::CH1ACR>;
#[doc = "Writer for register CH1ACR"]
pub type W = crate::W<u32, super::CH1ACR>;
#[doc = "Register CH1ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1ACV`"]
pub type CH1ACV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1ACV`"]
pub struct CH1ACV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1ACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH1ACV"]
    #[inline(always)]
    pub fn ch1acv(&self) -> CH1ACV_R {
        CH1ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1ACV"]
    #[inline(always)]
    pub fn ch1acv(&mut self) -> CH1ACV_W {
        CH1ACV_W { w: self }
    }
}
