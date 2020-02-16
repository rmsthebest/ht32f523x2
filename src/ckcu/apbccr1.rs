#[doc = "Reader of register APBCCR1"]
pub type R = crate::R<u32, super::APBCCR1>;
#[doc = "Writer for register APBCCR1"]
pub type W = crate::W<u32, super::APBCCR1>;
#[doc = "Register APBCCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APBCCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCTM0EN`"]
pub type MCTM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCTM0EN`"]
pub struct MCTM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTM0EN_W<'a> {
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
#[doc = "Reader of field `WDTEN`"]
pub type WDTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTEN`"]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
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
#[doc = "Reader of field `BKPREN`"]
pub type BKPREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPREN`"]
pub struct BKPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPREN_W<'a> {
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
#[doc = "Reader of field `GPTM0EN`"]
pub type GPTM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM0EN`"]
pub struct GPTM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM0EN_W<'a> {
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
#[doc = "Reader of field `GPTM1EN`"]
pub type GPTM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM1EN`"]
pub struct GPTM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM1EN_W<'a> {
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
#[doc = "Reader of field `BFTM0EN`"]
pub type BFTM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTM0EN`"]
pub struct BFTM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM0EN_W<'a> {
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
#[doc = "Reader of field `BFTM1EN`"]
pub type BFTM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTM1EN`"]
pub struct BFTM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM1EN_W<'a> {
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
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SCTM0EN`"]
pub type SCTM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCTM0EN`"]
pub struct SCTM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTM0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SCTM1EN`"]
pub type SCTM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCTM1EN`"]
pub struct SCTM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTM1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MCTM0EN"]
    #[inline(always)]
    pub fn mctm0en(&self) -> MCTM0EN_R {
        MCTM0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BKPREN"]
    #[inline(always)]
    pub fn bkpren(&self) -> BKPREN_R {
        BKPREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&self) -> GPTM0EN_R {
        GPTM0EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&self) -> GPTM1EN_R {
        GPTM1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    pub fn bftm0en(&self) -> BFTM0EN_R {
        BFTM0EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    pub fn bftm1en(&self) -> BFTM1EN_R {
        BFTM1EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    pub fn sctm0en(&self) -> SCTM0EN_R {
        SCTM0EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    pub fn sctm1en(&self) -> SCTM1EN_R {
        SCTM1EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTM0EN"]
    #[inline(always)]
    pub fn mctm0en(&mut self) -> MCTM0EN_W {
        MCTM0EN_W { w: self }
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
    #[doc = "Bit 6 - BKPREN"]
    #[inline(always)]
    pub fn bkpren(&mut self) -> BKPREN_W {
        BKPREN_W { w: self }
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&mut self) -> GPTM0EN_W {
        GPTM0EN_W { w: self }
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&mut self) -> GPTM1EN_W {
        GPTM1EN_W { w: self }
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    pub fn bftm0en(&mut self) -> BFTM0EN_W {
        BFTM0EN_W { w: self }
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    pub fn bftm1en(&mut self) -> BFTM1EN_W {
        BFTM1EN_W { w: self }
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    pub fn sctm0en(&mut self) -> SCTM0EN_W {
        SCTM0EN_W { w: self }
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    pub fn sctm1en(&mut self) -> SCTM1EN_W {
        SCTM1EN_W { w: self }
    }
}
