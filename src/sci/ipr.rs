#[doc = "Reader of register IPR"]
pub type R = crate::R<u32, super::IPR>;
#[doc = "Writer for register IPR"]
pub type W = crate::W<u32, super::IPR>;
#[doc = "Register IPR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PARP`"]
pub type PARP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARP`"]
pub struct PARP_W<'a> {
    w: &'a mut W,
}
impl<'a> PARP_W<'a> {
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
#[doc = "Reader of field `RXCP`"]
pub type RXCP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCP`"]
pub struct RXCP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCP_W<'a> {
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
#[doc = "Reader of field `TXCP`"]
pub type TXCP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCP`"]
pub struct TXCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCP_W<'a> {
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
#[doc = "Reader of field `WTP`"]
pub type WTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTP`"]
pub struct WTP_W<'a> {
    w: &'a mut W,
}
impl<'a> WTP_W<'a> {
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
#[doc = "Reader of field `CARDIRP`"]
pub type CARDIRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDIRP`"]
pub struct CARDIRP_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDIRP_W<'a> {
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
#[doc = "Reader of field `TXBEP`"]
pub type TXBEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBEP`"]
pub struct TXBEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBEP_W<'a> {
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
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    pub fn parp(&self) -> PARP_R {
        PARP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    pub fn rxcp(&self) -> RXCP_R {
        RXCP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    pub fn txcp(&self) -> TXCP_R {
        TXCP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    pub fn wtp(&self) -> WTP_R {
        WTP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    pub fn cardirp(&self) -> CARDIRP_R {
        CARDIRP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    pub fn txbep(&self) -> TXBEP_R {
        TXBEP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    pub fn parp(&mut self) -> PARP_W {
        PARP_W { w: self }
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    pub fn rxcp(&mut self) -> RXCP_W {
        RXCP_W { w: self }
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    pub fn txcp(&mut self) -> TXCP_W {
        TXCP_W { w: self }
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    pub fn wtp(&mut self) -> WTP_W {
        WTP_W { w: self }
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    pub fn cardirp(&mut self) -> CARDIRP_W {
        CARDIRP_W { w: self }
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    pub fn txbep(&mut self) -> TXBEP_W {
        TXBEP_W { w: self }
    }
}
