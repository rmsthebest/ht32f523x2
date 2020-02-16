#[doc = "Reader of register AHBCCR"]
pub type R = crate::R<u32, super::AHBCCR>;
#[doc = "Writer for register AHBCCR"]
pub type W = crate::W<u32, super::AHBCCR>;
#[doc = "Register AHBCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMCEN`"]
pub type FMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMCEN`"]
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
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
#[doc = "Reader of field `SRAMEN`"]
pub type SRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMEN`"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
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
#[doc = "Reader of field `PDMAEN`"]
pub type PDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMAEN`"]
pub struct PDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMAEN_W<'a> {
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
#[doc = "Reader of field `BMEN`"]
pub type BMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMEN`"]
pub struct BMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMEN_W<'a> {
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
#[doc = "Reader of field `APBEN`"]
pub type APBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APBEN`"]
pub struct APBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APBEN_W<'a> {
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
#[doc = "Reader of field `USBEN`"]
pub type USBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBEN`"]
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
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
#[doc = "Reader of field `CKREFEN`"]
pub type CKREFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKREFEN`"]
pub struct CKREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKREFEN_W<'a> {
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
#[doc = "Reader of field `EBIEN`"]
pub type EBIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIEN`"]
pub struct EBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIEN_W<'a> {
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
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Reader of field `PAEN`"]
pub type PAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAEN`"]
pub struct PAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAEN_W<'a> {
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
#[doc = "Reader of field `PBEN`"]
pub type PBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBEN`"]
pub struct PBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBEN_W<'a> {
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
#[doc = "Reader of field `PCEN`"]
pub type PCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCEN`"]
pub struct PCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEN_W<'a> {
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
#[doc = "Reader of field `PDEN`"]
pub type PDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN`"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    pub fn apben(&self) -> APBEN_R {
        APBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    pub fn ckrefen(&self) -> CKREFEN_R {
        CKREFEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    pub fn ebien(&self) -> EBIEN_R {
        EBIEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&mut self) -> PDMAEN_W {
        PDMAEN_W { w: self }
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&mut self) -> BMEN_W {
        BMEN_W { w: self }
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    pub fn apben(&mut self) -> APBEN_W {
        APBEN_W { w: self }
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    pub fn ckrefen(&mut self) -> CKREFEN_W {
        CKREFEN_W { w: self }
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    pub fn ebien(&mut self) -> EBIEN_W {
        EBIEN_W { w: self }
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&mut self) -> PAEN_W {
        PAEN_W { w: self }
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&mut self) -> PBEN_W {
        PBEN_W { w: self }
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W { w: self }
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
}
