#[doc = "Reader of register USART_USRSIFR"]
pub type R = crate::R<u32, super::USART_USRSIFR>;
#[doc = "Writer for register USART_USRSIFR"]
pub type W = crate::W<u32, super::USART_USRSIFR>;
#[doc = "Register USART_USRSIFR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_USRSIFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDNE`"]
pub type RXDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDNE`"]
pub struct RXDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDNE_W<'a> {
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
#[doc = "Reader of field `RXTOF`"]
pub type RXTOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTOF`"]
pub struct RXTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOF_W<'a> {
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
#[doc = "Reader of field `RSADDE`"]
pub type RSADDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSADDE`"]
pub struct RSADDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSADDE_W<'a> {
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
#[doc = "Reader of field `CTSC`"]
pub type CTSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSC`"]
pub struct CTSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSC_W<'a> {
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
#[doc = "Reader of field `CTSS`"]
pub type CTSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSS`"]
pub struct CTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RXDNE_R {
        RXDNE_R::new((self.bits & 0x01) != 0)
    }
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
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    pub fn rxtof(&self) -> RXTOF_R {
        RXTOF_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    pub fn rsadde(&self) -> RSADDE_R {
        RSADDE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    pub fn ctsc(&self) -> CTSC_R {
        CTSC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    pub fn ctss(&self) -> CTSS_R {
        CTSS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&mut self) -> RXDNE_W {
        RXDNE_W { w: self }
    }
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
    #[doc = "Bit 6 - RXTOF"]
    #[inline(always)]
    pub fn rxtof(&mut self) -> RXTOF_W {
        RXTOF_W { w: self }
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
    #[doc = "Bit 9 - RSADDE"]
    #[inline(always)]
    pub fn rsadde(&mut self) -> RSADDE_W {
        RSADDE_W { w: self }
    }
    #[doc = "Bit 10 - CTSC"]
    #[inline(always)]
    pub fn ctsc(&mut self) -> CTSC_W {
        CTSC_W { w: self }
    }
    #[doc = "Bit 11 - CTSS"]
    #[inline(always)]
    pub fn ctss(&mut self) -> CTSS_W {
        CTSS_W { w: self }
    }
}
