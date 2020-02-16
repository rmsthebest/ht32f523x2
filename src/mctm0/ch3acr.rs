#[doc = "Reader of register CH3ACR"]
pub type R = crate::R<u32, super::CH3ACR>;
#[doc = "Writer for register CH3ACR"]
pub type W = crate::W<u32, super::CH3ACR>;
#[doc = "Register CH3ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3ACV`"]
pub type CH3ACV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH3ACV`"]
pub struct CH3ACV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3ACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    pub fn ch3acv(&self) -> CH3ACV_R {
        CH3ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3ACV"]
    #[inline(always)]
    pub fn ch3acv(&mut self) -> CH3ACV_W {
        CH3ACV_W { w: self }
    }
}
