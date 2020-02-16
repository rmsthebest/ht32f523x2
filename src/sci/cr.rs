#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONV`"]
pub type CONV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONV`"]
pub struct CONV_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_W<'a> {
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
#[doc = "Reader of field `CREP`"]
pub type CREP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CREP`"]
pub struct CREP_W<'a> {
    w: &'a mut W,
}
impl<'a> CREP_W<'a> {
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
#[doc = "Reader of field `WTEN`"]
pub type WTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTEN`"]
pub struct WTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WTEN_W<'a> {
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
#[doc = "Reader of field `SCIM`"]
pub type SCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIM`"]
pub struct SCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIM_W<'a> {
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
#[doc = "Reader of field `RETRY`"]
pub type RETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRY`"]
pub struct RETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRY_W<'a> {
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
#[doc = "Reader of field `ENSCI`"]
pub type ENSCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSCI`"]
pub struct ENSCI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSCI_W<'a> {
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
#[doc = "Reader of field `DETCNF`"]
pub type DETCNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DETCNF`"]
pub struct DETCNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DETCNF_W<'a> {
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
#[doc = "Reader of field `TXDMA`"]
pub type TXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMA`"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
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
#[doc = "Reader of field `RXDMA`"]
pub type RXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMA`"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    pub fn wten(&self) -> WTEN_R {
        WTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    pub fn scim(&self) -> SCIM_R {
        SCIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    pub fn ensci(&self) -> ENSCI_R {
        ENSCI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    pub fn detcnf(&self) -> DETCNF_R {
        DETCNF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CONV"]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W { w: self }
    }
    #[doc = "Bit 1 - CREP"]
    #[inline(always)]
    pub fn crep(&mut self) -> CREP_W {
        CREP_W { w: self }
    }
    #[doc = "Bit 2 - WTEN"]
    #[inline(always)]
    pub fn wten(&mut self) -> WTEN_W {
        WTEN_W { w: self }
    }
    #[doc = "Bit 3 - SCIM"]
    #[inline(always)]
    pub fn scim(&mut self) -> SCIM_W {
        SCIM_W { w: self }
    }
    #[doc = "Bit 4 - RETRY"]
    #[inline(always)]
    pub fn retry(&mut self) -> RETRY_W {
        RETRY_W { w: self }
    }
    #[doc = "Bit 5 - ENSCI"]
    #[inline(always)]
    pub fn ensci(&mut self) -> ENSCI_W {
        ENSCI_W { w: self }
    }
    #[doc = "Bit 6 - DETCNF"]
    #[inline(always)]
    pub fn detcnf(&mut self) -> DETCNF_W {
        DETCNF_W { w: self }
    }
    #[doc = "Bit 8 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 9 - RXDMA"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
}
