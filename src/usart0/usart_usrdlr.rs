#[doc = "Reader of register USART_USRDLR"]
pub type R = crate::R<u32, super::USART_USRDLR>;
#[doc = "Writer for register USART_USRDLR"]
pub type W = crate::W<u32, super::USART_USRDLR>;
#[doc = "Register USART_USRDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRDLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRD`"]
pub type BRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRD`"]
pub struct BRD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    pub fn brd(&self) -> BRD_R {
        BRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    pub fn brd(&mut self) -> BRD_W {
        BRD_W { w: self }
    }
}
