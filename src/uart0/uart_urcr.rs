#[doc = "Reader of register UART_URCR"]
pub type R = crate::R<u32, super::UART_URCR>;
#[doc = "Writer for register UART_URCR"]
pub type W = crate::W<u32, super::UART_URCR>;
#[doc = "Register UART_URCR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_URCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRSM`"]
pub type TRSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRSM`"]
pub struct TRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSM_W<'a> {
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
#[doc = "Reader of field `URTXEN`"]
pub type URTXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URTXEN`"]
pub struct URTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URTXEN_W<'a> {
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
#[doc = "Reader of field `URRXEN`"]
pub type URRXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URRXEN`"]
pub struct URRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URRXEN_W<'a> {
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
#[doc = "Reader of field `TXDMAEN`"]
pub type TXDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMAEN`"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
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
#[doc = "Reader of field `RXDMAEN`"]
pub type RXDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAEN`"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `NSB`"]
pub type NSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSB`"]
pub struct NSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSB_W<'a> {
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
#[doc = "Reader of field `PBE`"]
pub type PBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBE`"]
pub struct PBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBE_W<'a> {
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
#[doc = "Reader of field `EPE`"]
pub type EPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPE`"]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BCB`"]
pub type BCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCB`"]
pub struct BCB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&self) -> TRSM_R {
        TRSM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    pub fn urtxen(&self) -> URTXEN_R {
        URTXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    pub fn urrxen(&self) -> URRXEN_R {
        URRXEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&mut self) -> TRSM_W {
        TRSM_W { w: self }
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    pub fn urtxen(&mut self) -> URTXEN_W {
        URTXEN_W { w: self }
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    pub fn urrxen(&mut self) -> URRXEN_W {
        URRXEN_W { w: self }
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    pub fn nsb(&mut self) -> NSB_W {
        NSB_W { w: self }
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    pub fn pbe(&mut self) -> PBE_W {
        PBE_W { w: self }
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    pub fn bcb(&mut self) -> BCB_W {
        BCB_W { w: self }
    }
}
