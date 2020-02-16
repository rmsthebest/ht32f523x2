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
#[doc = "Reader of field `TXBE`"]
pub type TXBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBE`"]
pub struct TXBE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBE_W<'a> {
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
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
#[doc = "Reader of field `RXBNE`"]
pub type RXBNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBNE`"]
pub struct RXBNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBNE_W<'a> {
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
#[doc = "Reader of field `WC`"]
pub type WC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WC`"]
pub struct WC_W<'a> {
    w: &'a mut W,
}
impl<'a> WC_W<'a> {
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
#[doc = "Reader of field `RO`"]
pub type RO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RO`"]
pub struct RO_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_W<'a> {
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
#[doc = "Reader of field `MF`"]
pub type MF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MF`"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
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
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
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
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TO`"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    pub fn txbe(&self) -> TXBE_R {
        TXBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    pub fn rxbne(&self) -> RXBNE_R {
        RXBNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    pub fn txbe(&mut self) -> TXBE_W {
        TXBE_W { w: self }
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    pub fn rxbne(&mut self) -> RXBNE_W {
        RXBNE_W { w: self }
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    pub fn wc(&mut self) -> WC_W {
        WC_W { w: self }
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W { w: self }
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
