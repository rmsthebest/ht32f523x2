#[doc = "Reader of register GTR"]
pub type R = crate::R<u32, super::GTR>;
#[doc = "Writer for register GTR"]
pub type W = crate::W<u32, super::GTR>;
#[doc = "Register GTR `reset()`'s with value 0"]
impl crate::ResetValue for super::GTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GT`"]
pub struct GT_W<'a> {
    w: &'a mut W,
}
impl<'a> GT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W { w: self }
    }
}
