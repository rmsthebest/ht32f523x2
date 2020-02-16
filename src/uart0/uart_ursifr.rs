#[doc = "Reader of register UART_URSIFR"]
pub type R = crate::R<u32, super::UART_URSIFR>;
#[doc = "Writer for register UART_URSIFR"]
pub type W = crate::W<u32, super::UART_URSIFR>;
#[doc = "Register UART_URSIFR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_URSIFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OEI`"]
pub type OEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEI`"]
pub struct OEI_W<'a> {
    w: &'a mut W,
}
impl<'a> OEI_W<'a> {
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
#[doc = "Reader of field `PEI`"]
pub type PEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEI`"]
pub struct PEI_W<'a> {
    w: &'a mut W,
}
impl<'a> PEI_W<'a> {
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
#[doc = "Reader of field `FEI`"]
pub type FEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEI`"]
pub struct FEI_W<'a> {
    w: &'a mut W,
}
impl<'a> FEI_W<'a> {
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
#[doc = "Reader of field `BII`"]
pub type BII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BII`"]
pub struct BII_W<'a> {
    w: &'a mut W,
}
impl<'a> BII_W<'a> {
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
#[doc = "Reader of field `RXDR`"]
pub type RXDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDR`"]
pub struct RXDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDR_W<'a> {
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
#[doc = "Reader of field `TXDE`"]
pub type TXDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDE`"]
pub struct TXDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDE_W<'a> {
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
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXC`"]
pub struct TXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_W<'a> {
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
impl R {
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OEI_R {
        OEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&self) -> PEI_R {
        PEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&self) -> FEI_R {
        FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&self) -> BII_R {
        BII_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&mut self) -> OEI_W {
        OEI_W { w: self }
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&mut self) -> PEI_W {
        PEI_W { w: self }
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&mut self) -> FEI_W {
        FEI_W { w: self }
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&mut self) -> BII_W {
        BII_W { w: self }
    }
    #[doc = "Bit 5 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RXDR_W {
        RXDR_W { w: self }
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&mut self) -> TXDE_W {
        TXDE_W { w: self }
    }
    #[doc = "Bit 8 - TXC"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
}
