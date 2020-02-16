#[doc = "Reader of register USART_USRIER"]
pub type R = crate::R<u32, super::USART_USRIER>;
#[doc = "Writer for register USART_USRIER"]
pub type W = crate::W<u32, super::USART_USRIER>;
#[doc = "Register USART_USRIER `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDRIE`"]
pub type RXDRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDRIE`"]
pub struct RXDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDRIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TXDEIE`"]
pub type TXDEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDEIE`"]
pub struct TXDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TXCIE`"]
pub type TXCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCIE`"]
pub struct TXCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `OEIE`"]
pub type OEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIE`"]
pub struct OEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PEIE`"]
pub type PEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIE`"]
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BIE`"]
pub type BIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIE`"]
pub struct BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RSADDIE`"]
pub type RSADDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSADDIE`"]
pub struct RSADDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSADDIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RXTOIE`"]
pub type RXTOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTOIE`"]
pub struct RXTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTSIE`"]
pub type CTSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSIE`"]
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RXDRIE_R {
        RXDRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TXDEIE_R {
        TXDEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    pub fn oeie(&self) -> OEIE_R {
        OEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RSADDIE"]
    #[inline(always)]
    pub fn rsaddie(&self) -> RSADDIE_R {
        RSADDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXTOIE"]
    #[inline(always)]
    pub fn rxtoie(&self) -> RXTOIE_R {
        RXTOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTSIE"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&mut self) -> RXDRIE_W {
        RXDRIE_W { w: self }
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&mut self) -> TXDEIE_W {
        TXDEIE_W { w: self }
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&mut self) -> TXCIE_W {
        TXCIE_W { w: self }
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    pub fn oeie(&mut self) -> OEIE_W {
        OEIE_W { w: self }
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W {
        BIE_W { w: self }
    }
    #[doc = "Bit 7 - RSADDIE"]
    #[inline(always)]
    pub fn rsaddie(&mut self) -> RSADDIE_W {
        RSADDIE_W { w: self }
    }
    #[doc = "Bit 8 - RXTOIE"]
    #[inline(always)]
    pub fn rxtoie(&mut self) -> RXTOIE_W {
        RXTOIE_W { w: self }
    }
    #[doc = "Bit 9 - CTSIE"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
}
