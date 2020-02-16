#[doc = "Reader of register DMAR"]
pub type R = crate::R<u32, super::DMAR>;
#[doc = "Writer for register DMAR"]
pub type W = crate::W<u32, super::DMAR>;
#[doc = "Register DMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDMAS`"]
pub type ADDMAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAS`"]
pub struct ADDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAS_W<'a> {
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
#[doc = "Reader of field `ADDMAG`"]
pub type ADDMAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAG`"]
pub struct ADDMAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAG_W<'a> {
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
#[doc = "Reader of field `ADDMAC`"]
pub type ADDMAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAC`"]
pub struct ADDMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&self) -> ADDMAS_R {
        ADDMAS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&self) -> ADDMAG_R {
        ADDMAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&self) -> ADDMAC_R {
        ADDMAC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&mut self) -> ADDMAS_W {
        ADDMAS_W { w: self }
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&mut self) -> ADDMAG_W {
        ADDMAG_W { w: self }
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&mut self) -> ADDMAC_W {
        ADDMAC_W { w: self }
    }
}
