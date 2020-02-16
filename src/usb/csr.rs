#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRES`"]
pub type FRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRES`"]
pub struct FRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRES_W<'a> {
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
#[doc = "Reader of field `PDWN`"]
pub type PDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDWN`"]
pub struct PDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWN_W<'a> {
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
#[doc = "Reader of field `LPMODE`"]
pub type LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPMODE`"]
pub struct LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMODE_W<'a> {
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
#[doc = "Reader of field `GENRSM`"]
pub type GENRSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GENRSM`"]
pub struct GENRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> GENRSM_W<'a> {
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
#[doc = "Reader of field `RXDP`"]
pub type RXDP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDP`"]
pub struct RXDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDP_W<'a> {
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
#[doc = "Reader of field `RXDM`"]
pub type RXDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDM`"]
pub struct RXDM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDM_W<'a> {
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
#[doc = "Reader of field `ADRSET`"]
pub type ADRSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRSET`"]
pub struct ADRSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRSET_W<'a> {
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
#[doc = "Reader of field `SRAMRSTC`"]
pub type SRAMRSTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMRSTC`"]
pub struct SRAMRSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMRSTC_W<'a> {
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
#[doc = "Reader of field `DPPUEN`"]
pub type DPPUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPPUEN`"]
pub struct DPPUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPUEN_W<'a> {
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
#[doc = "Reader of field `DPWKEN`"]
pub type DPWKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPWKEN`"]
pub struct DPWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPWKEN_W<'a> {
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
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    pub fn genrsm(&self) -> GENRSM_R {
        GENRSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    pub fn adrset(&self) -> ADRSET_R {
        ADRSET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    pub fn sramrstc(&self) -> SRAMRSTC_R {
        SRAMRSTC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    pub fn dppuen(&self) -> DPPUEN_R {
        DPPUEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    pub fn dpwken(&self) -> DPWKEN_R {
        DPWKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W {
        FRES_W { w: self }
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W {
        PDWN_W { w: self }
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W { w: self }
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    pub fn genrsm(&mut self) -> GENRSM_W {
        GENRSM_W { w: self }
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&mut self) -> RXDP_W {
        RXDP_W { w: self }
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&mut self) -> RXDM_W {
        RXDM_W { w: self }
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    pub fn adrset(&mut self) -> ADRSET_W {
        ADRSET_W { w: self }
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    pub fn sramrstc(&mut self) -> SRAMRSTC_W {
        SRAMRSTC_W { w: self }
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    pub fn dppuen(&mut self) -> DPPUEN_W {
        DPPUEN_W { w: self }
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    pub fn dpwken(&mut self) -> DPWKEN_W {
        DPWKEN_W { w: self }
    }
}
