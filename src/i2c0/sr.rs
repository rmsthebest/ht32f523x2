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
#[doc = "Reader of field `STA`"]
pub type STA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STA`"]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
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
#[doc = "Reader of field `STO`"]
pub type STO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STO`"]
pub struct STO_W<'a> {
    w: &'a mut W,
}
impl<'a> STO_W<'a> {
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
#[doc = "Reader of field `ADRS`"]
pub type ADRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRS`"]
pub struct ADRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRS_W<'a> {
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
#[doc = "Reader of field `GCS`"]
pub type GCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCS`"]
pub struct GCS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCS_W<'a> {
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
#[doc = "Reader of field `ARBLOS`"]
pub type ARBLOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOS`"]
pub struct ARBLOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOS_W<'a> {
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
#[doc = "Reader of field `RXNACK`"]
pub type RXNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXNACK`"]
pub struct RXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNACK_W<'a> {
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
#[doc = "Reader of field `BUSERR`"]
pub type BUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERR`"]
pub struct BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERR_W<'a> {
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
#[doc = "Reader of field `TOUTF`"]
pub type TOUTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUTF`"]
pub struct TOUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RXBF`"]
pub type RXBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBF`"]
pub struct RXBF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBF_W<'a> {
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
#[doc = "Reader of field `BUSBUSY`"]
pub type BUSBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSBUSY`"]
pub struct BUSBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TXNRX`"]
pub type TXNRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXNRX`"]
pub struct TXNRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    pub fn adrs(&self) -> ADRS_R {
        ADRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    pub fn arblos(&self) -> ARBLOS_R {
        ARBLOS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    pub fn toutf(&self) -> TOUTF_R {
        TOUTF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RXDNE_R {
        RXDNE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    pub fn rxbf(&self) -> RXBF_R {
        RXBF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&self) -> BUSBUSY_R {
        BUSBUSY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    pub fn txnrx(&self) -> TXNRX_R {
        TXNRX_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W { w: self }
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    pub fn adrs(&mut self) -> ADRS_W {
        ADRS_W { w: self }
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W {
        GCS_W { w: self }
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    pub fn arblos(&mut self) -> ARBLOS_W {
        ARBLOS_W { w: self }
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    pub fn rxnack(&mut self) -> RXNACK_W {
        RXNACK_W { w: self }
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W { w: self }
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    pub fn toutf(&mut self) -> TOUTF_W {
        TOUTF_W { w: self }
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&mut self) -> RXDNE_W {
        RXDNE_W { w: self }
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    pub fn txde(&mut self) -> TXDE_W {
        TXDE_W { w: self }
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    pub fn rxbf(&mut self) -> RXBF_W {
        RXBF_W { w: self }
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&mut self) -> BUSBUSY_W {
        BUSBUSY_W { w: self }
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    pub fn txnrx(&mut self) -> TXNRX_W {
        TXNRX_W { w: self }
    }
}
