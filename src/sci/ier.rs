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
#[doc = "Reader of field `PARE`"]
pub type PARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARE`"]
pub struct PARE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARE_W<'a> {
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
#[doc = "Reader of field `RXCE`"]
pub type RXCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCE`"]
pub struct RXCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCE_W<'a> {
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
#[doc = "Reader of field `TXCE`"]
pub type TXCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCE`"]
pub struct TXCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCE_W<'a> {
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
#[doc = "Reader of field `WTE`"]
pub type WTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTE`"]
pub struct WTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTE_W<'a> {
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
#[doc = "Reader of field `CARDIRE`"]
pub type CARDIRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDIRE`"]
pub struct CARDIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDIRE_W<'a> {
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
#[doc = "Reader of field `TXBEE`"]
pub type TXBEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBEE`"]
pub struct TXBEE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBEE_W<'a> {
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
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    pub fn rxce(&self) -> RXCE_R {
        RXCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    pub fn txce(&self) -> TXCE_R {
        TXCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    pub fn cardire(&self) -> CARDIRE_R {
        CARDIRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    pub fn txbee(&self) -> TXBEE_R {
        TXBEE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    pub fn pare(&mut self) -> PARE_W {
        PARE_W { w: self }
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    pub fn rxce(&mut self) -> RXCE_W {
        RXCE_W { w: self }
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    pub fn txce(&mut self) -> TXCE_W {
        TXCE_W { w: self }
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W {
        WTE_W { w: self }
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    pub fn cardire(&mut self) -> CARDIRE_W {
        CARDIRE_W { w: self }
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    pub fn txbee(&mut self) -> TXBEE_W {
        TXBEE_W { w: self }
    }
}
