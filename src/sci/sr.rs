#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PARF`"]
pub type PARF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARF`"]
pub struct PARF_W<'a> {
    w: &'a mut W,
}
impl<'a> PARF_W<'a> {
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
#[doc = "Reader of field `RXCF`"]
pub type RXCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCF`"]
pub struct RXCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCF_W<'a> {
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
#[doc = "Reader of field `TXCF`"]
pub type TXCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCF`"]
pub struct TXCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCF_W<'a> {
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
#[doc = "Reader of field `WTF`"]
pub type WTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTF`"]
pub struct WTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTF_W<'a> {
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
#[doc = "Reader of field `CPREF`"]
pub type CPREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPREF`"]
pub struct CPREF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPREF_W<'a> {
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
#[doc = "Reader of field `TXBEF`"]
pub type TXBEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBEF`"]
pub struct TXBEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBEF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    pub fn rxcf(&self) -> RXCF_R {
        RXCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    pub fn txcf(&self) -> TXCF_R {
        TXCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    pub fn wtf(&self) -> WTF_R {
        WTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    pub fn cpref(&self) -> CPREF_R {
        CPREF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    pub fn txbef(&self) -> TXBEF_R {
        TXBEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    pub fn parf(&mut self) -> PARF_W {
        PARF_W { w: self }
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    pub fn rxcf(&mut self) -> RXCF_W {
        RXCF_W { w: self }
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    pub fn txcf(&mut self) -> TXCF_W {
        TXCF_W { w: self }
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    pub fn wtf(&mut self) -> WTF_W {
        WTF_W { w: self }
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    pub fn cpref(&mut self) -> CPREF_W {
        CPREF_W { w: self }
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    pub fn txbef(&mut self) -> TXBEF_W {
        TXBEF_W { w: self }
    }
}
