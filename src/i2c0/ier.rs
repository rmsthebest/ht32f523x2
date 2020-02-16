#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STAIE`"]
pub type STAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAIE`"]
pub struct STAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STAIE_W<'a> {
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
#[doc = "Reader of field `STOIE`"]
pub type STOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOIE`"]
pub struct STOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOIE_W<'a> {
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
#[doc = "Reader of field `ADRSIE`"]
pub type ADRSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRSIE`"]
pub struct ADRSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRSIE_W<'a> {
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
#[doc = "Reader of field `GCSIE`"]
pub type GCSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCSIE`"]
pub struct GCSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCSIE_W<'a> {
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
#[doc = "Reader of field `ARBLOSIE`"]
pub type ARBLOSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOSIE`"]
pub struct ARBLOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOSIE_W<'a> {
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
#[doc = "Reader of field `RXNACKIE`"]
pub type RXNACKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXNACKIE`"]
pub struct RXNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNACKIE_W<'a> {
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
#[doc = "Reader of field `BUSERRIE`"]
pub type BUSERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERRIE`"]
pub struct BUSERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TOUTIE`"]
pub type TOUTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUTIE`"]
pub struct TOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RXDNEIE`"]
pub type RXDNEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDNEIE`"]
pub struct RXDNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDNEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RXBFIE`"]
pub type RXBFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBFIE`"]
pub struct RXBFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    pub fn staie(&self) -> STAIE_R {
        STAIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    pub fn stoie(&self) -> STOIE_R {
        STOIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    pub fn adrsie(&self) -> ADRSIE_R {
        ADRSIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    pub fn gcsie(&self) -> GCSIE_R {
        GCSIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    pub fn arblosie(&self) -> ARBLOSIE_R {
        ARBLOSIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    pub fn rxnackie(&self) -> RXNACKIE_R {
        RXNACKIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    pub fn buserrie(&self) -> BUSERRIE_R {
        BUSERRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    pub fn toutie(&self) -> TOUTIE_R {
        TOUTIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    pub fn rxdneie(&self) -> RXDNEIE_R {
        RXDNEIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TXDEIE_R {
        TXDEIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    pub fn staie(&mut self) -> STAIE_W {
        STAIE_W { w: self }
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    pub fn stoie(&mut self) -> STOIE_W {
        STOIE_W { w: self }
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    pub fn adrsie(&mut self) -> ADRSIE_W {
        ADRSIE_W { w: self }
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    pub fn gcsie(&mut self) -> GCSIE_W {
        GCSIE_W { w: self }
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    pub fn arblosie(&mut self) -> ARBLOSIE_W {
        ARBLOSIE_W { w: self }
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    pub fn rxnackie(&mut self) -> RXNACKIE_W {
        RXNACKIE_W { w: self }
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    pub fn buserrie(&mut self) -> BUSERRIE_W {
        BUSERRIE_W { w: self }
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    pub fn toutie(&mut self) -> TOUTIE_W {
        TOUTIE_W { w: self }
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    pub fn rxdneie(&mut self) -> RXDNEIE_W {
        RXDNEIE_W { w: self }
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&mut self) -> TXDEIE_W {
        TXDEIE_W { w: self }
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    pub fn rxbfie(&mut self) -> RXBFIE_W {
        RXBFIE_W { w: self }
    }
}
