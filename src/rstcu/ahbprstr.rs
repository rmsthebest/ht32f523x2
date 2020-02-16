#[doc = "Reader of register AHBPRSTR"]
pub type R = crate::R<u32, super::AHBPRSTR>;
#[doc = "Writer for register AHBPRSTR"]
pub type W = crate::W<u32, super::AHBPRSTR>;
#[doc = "Register AHBPRSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBPRSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMARST`"]
pub type DMARST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARST`"]
pub struct DMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARST_W<'a> {
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
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Reader of field `EBIRST`"]
pub type EBIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIRST`"]
pub struct EBIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIRST_W<'a> {
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
#[doc = "Reader of field `CRCRST`"]
pub type CRCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCRST`"]
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
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
#[doc = "Reader of field `PARST`"]
pub type PARST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARST`"]
pub struct PARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PARST_W<'a> {
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
#[doc = "Reader of field `PBRST`"]
pub type PBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBRST`"]
pub struct PBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PBRST_W<'a> {
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
#[doc = "Reader of field `PCRST`"]
pub type PCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCRST`"]
pub struct PCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRST_W<'a> {
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
#[doc = "Reader of field `PDRST`"]
pub type PDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDRST`"]
pub struct PDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDRST_W<'a> {
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
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EBIRST"]
    #[inline(always)]
    pub fn ebirst(&self) -> EBIRST_R {
        EBIRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W {
        DMARST_W { w: self }
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 6 - EBIRST"]
    #[inline(always)]
    pub fn ebirst(&mut self) -> EBIRST_W {
        EBIRST_W { w: self }
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    pub fn parst(&mut self) -> PARST_W {
        PARST_W { w: self }
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PBRST_W {
        PBRST_W { w: self }
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PCRST_W {
        PCRST_W { w: self }
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&mut self) -> PDRST_W {
        PDRST_W { w: self }
    }
}
