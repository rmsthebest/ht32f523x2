#[doc = "Reader of register CH0ACR"]
pub type R = crate::R<u32, super::CH0ACR>;
#[doc = "Writer for register CH0ACR"]
pub type W = crate::W<u32, super::CH0ACR>;
#[doc = "Register CH0ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0ACV`"]
pub type CH0ACV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH0ACV`"]
pub struct CH0ACV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0ACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    pub fn ch0acv(&self) -> CH0ACV_R {
        CH0ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    pub fn ch0acv(&mut self) -> CH0ACV_W {
        CH0ACV_W { w: self }
    }
}
