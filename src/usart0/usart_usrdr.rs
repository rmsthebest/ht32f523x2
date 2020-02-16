#[doc = "Reader of register USART_USRDR"]
pub type R = crate::R<u32, super::USART_USRDR>;
#[doc = "Writer for register USART_USRDR"]
pub type W = crate::W<u32, super::USART_USRDR>;
#[doc = "Register USART_USRDR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DB`"]
pub type DB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DB`"]
pub struct DB_W<'a> {
    w: &'a mut W,
}
impl<'a> DB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    pub fn db(&mut self) -> DB_W {
        DB_W { w: self }
    }
}
