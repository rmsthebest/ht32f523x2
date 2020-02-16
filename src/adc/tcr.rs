#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSW`"]
pub type ADSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADSW`"]
pub struct ADSW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSW_W<'a> {
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
#[doc = "Reader of field `ADEXTI`"]
pub type ADEXTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEXTI`"]
pub struct ADEXTI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEXTI_W<'a> {
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
#[doc = "Reader of field `TM`"]
pub type TM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM`"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
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
#[doc = "Reader of field `BFTM`"]
pub type BFTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTM`"]
pub struct BFTM_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM_W<'a> {
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
#[doc = "Reader of field `CMP`"]
pub type CMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP`"]
pub struct CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> ADSW_R {
        ADSW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> ADEXTI_R {
        ADEXTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    pub fn bftm(&self) -> BFTM_R {
        BFTM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&mut self) -> ADSW_W {
        ADSW_W { w: self }
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&mut self) -> ADEXTI_W {
        ADEXTI_W { w: self }
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    pub fn bftm(&mut self) -> BFTM_W {
        BFTM_W { w: self }
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W { w: self }
    }
}
