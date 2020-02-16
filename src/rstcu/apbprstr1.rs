#[doc = "Reader of register APBPRSTR1"]
pub type R = crate::R<u32, super::APBPRSTR1>;
#[doc = "Writer for register APBPRSTR1"]
pub type W = crate::W<u32, super::APBPRSTR1>;
#[doc = "Register APBPRSTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APBPRSTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCTM0RST`"]
pub type MCTM0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCTM0RST`"]
pub struct MCTM0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTM0RST_W<'a> {
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
#[doc = "Reader of field `WDTRST`"]
pub type WDTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRST`"]
pub struct WDTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRST_W<'a> {
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
#[doc = "Reader of field `GPTM0RST`"]
pub type GPTM0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM0RST`"]
pub struct GPTM0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM0RST_W<'a> {
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
#[doc = "Reader of field `GPTM1RST`"]
pub type GPTM1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM1RST`"]
pub struct GPTM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM1RST_W<'a> {
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
#[doc = "Reader of field `BFTM0RST`"]
pub type BFTM0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTM0RST`"]
pub struct BFTM0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM0RST_W<'a> {
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
#[doc = "Reader of field `BFTM1RST`"]
pub type BFTM1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFTM1RST`"]
pub struct BFTM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BFTM1RST_W<'a> {
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
#[doc = "Reader of field `CMPRST`"]
pub type CMPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPRST`"]
pub struct CMPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPRST_W<'a> {
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
#[doc = "Reader of field `ADCRST`"]
pub type ADCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCRST`"]
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
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
#[doc = "Reader of field `SCTM0RST`"]
pub type SCTM0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCTM0RST`"]
pub struct SCTM0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTM0RST_W<'a> {
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
#[doc = "Reader of field `SCTM1RST`"]
pub type SCTM1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCTM1RST`"]
pub struct SCTM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTM1RST_W<'a> {
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
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    pub fn mctm0rst(&self) -> MCTM0RST_R {
        MCTM0RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&self) -> WDTRST_R {
        WDTRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    pub fn gptm0rst(&self) -> GPTM0RST_R {
        GPTM0RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    pub fn gptm1rst(&self) -> GPTM1RST_R {
        GPTM1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    pub fn bftm0rst(&self) -> BFTM0RST_R {
        BFTM0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    pub fn bftm1rst(&self) -> BFTM1RST_R {
        BFTM1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    pub fn cmprst(&self) -> CMPRST_R {
        CMPRST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    pub fn sctm0rst(&self) -> SCTM0RST_R {
        SCTM0RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    pub fn sctm1rst(&self) -> SCTM1RST_R {
        SCTM1RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    pub fn mctm0rst(&mut self) -> MCTM0RST_W {
        MCTM0RST_W { w: self }
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&mut self) -> WDTRST_W {
        WDTRST_W { w: self }
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    pub fn gptm0rst(&mut self) -> GPTM0RST_W {
        GPTM0RST_W { w: self }
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    pub fn gptm1rst(&mut self) -> GPTM1RST_W {
        GPTM1RST_W { w: self }
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    pub fn bftm0rst(&mut self) -> BFTM0RST_W {
        BFTM0RST_W { w: self }
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    pub fn bftm1rst(&mut self) -> BFTM1RST_W {
        BFTM1RST_W { w: self }
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    pub fn cmprst(&mut self) -> CMPRST_W {
        CMPRST_W { w: self }
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    pub fn sctm0rst(&mut self) -> SCTM0RST_W {
        SCTM0RST_W { w: self }
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    pub fn sctm1rst(&mut self) -> SCTM1RST_W {
        SCTM1RST_W { w: self }
    }
}
