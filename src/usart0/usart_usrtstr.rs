#[doc = "Reader of register USART_USRTSTR"]
pub type R = crate::R<u32, super::USART_USRTSTR>;
#[doc = "Writer for register USART_USRTSTR"]
pub type W = crate::W<u32, super::USART_USRTSTR>;
#[doc = "Register USART_USRTSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRTSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LBM`"]
pub type LBM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LBM`"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
}
