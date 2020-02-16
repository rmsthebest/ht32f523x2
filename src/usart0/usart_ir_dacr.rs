#[doc = "Reader of register USART_IrDACR"]
pub type R = crate::R<u32, super::USART_IRDACR>;
#[doc = "Writer for register USART_IrDACR"]
pub type W = crate::W<u32, super::USART_IRDACR>;
#[doc = "Register USART_IrDACR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_IRDACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IrDAEN`"]
pub type IRDAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IrDAEN`"]
pub struct IRDAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAEN_W<'a> {
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
#[doc = "Reader of field `IrDALP`"]
pub type IRDALP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IrDALP`"]
pub struct IRDALP_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDALP_W<'a> {
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
#[doc = "Reader of field `TXSEL`"]
pub type TXSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSEL`"]
pub struct TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEL_W<'a> {
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
#[doc = "Reader of field `LB`"]
pub type LB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LB`"]
pub struct LB_W<'a> {
    w: &'a mut W,
}
impl<'a> LB_W<'a> {
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
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
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
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
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
#[doc = "Reader of field `IrDAPSC`"]
pub type IRDAPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IrDAPSC`"]
pub struct IRDAPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    pub fn ir_daen(&self) -> IRDAEN_R {
        IRDAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    pub fn ir_dalp(&self) -> IRDALP_R {
        IRDALP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&self) -> TXSEL_R {
        TXSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    pub fn ir_dapsc(&self) -> IRDAPSC_R {
        IRDAPSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    pub fn ir_daen(&mut self) -> IRDAEN_W {
        IRDAEN_W { w: self }
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    pub fn ir_dalp(&mut self) -> IRDALP_W {
        IRDALP_W { w: self }
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&mut self) -> TXSEL_W {
        TXSEL_W { w: self }
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    pub fn lb(&mut self) -> LB_W {
        LB_W { w: self }
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    pub fn ir_dapsc(&mut self) -> IRDAPSC_W {
        IRDAPSC_W { w: self }
    }
}
