#[doc = "Reader of register GPTM_CH2OCFR"]
pub type R = crate::R<u32, super::GPTM_CH2OCFR>;
#[doc = "Writer for register GPTM_CH2OCFR"]
pub type W = crate::W<u32, super::GPTM_CH2OCFR>;
#[doc = "Register GPTM_CH2OCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CH2OCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2OM`"]
pub type CH2OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2OM`"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CH2PRE`"]
pub type CH2PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2PRE`"]
pub struct CH2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PRE_W<'a> {
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
#[doc = "Reader of field `CH2IMAE`"]
pub type CH2IMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2IMAE`"]
pub struct CH2IMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2IMAE_W<'a> {
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
#[doc = "Reader of field `CH2OM3`"]
pub type CH2OM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OM3`"]
pub struct CH2OM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM3_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    pub fn ch2pre(&self) -> CH2PRE_R {
        CH2PRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    pub fn ch2imae(&self) -> CH2IMAE_R {
        CH2IMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    pub fn ch2om3(&self) -> CH2OM3_R {
        CH2OM3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 4 - CH2PRE"]
    #[inline(always)]
    pub fn ch2pre(&mut self) -> CH2PRE_W {
        CH2PRE_W { w: self }
    }
    #[doc = "Bit 5 - CH2IMAE"]
    #[inline(always)]
    pub fn ch2imae(&mut self) -> CH2IMAE_W {
        CH2IMAE_W { w: self }
    }
    #[doc = "Bit 8 - CH2OM3"]
    #[inline(always)]
    pub fn ch2om3(&mut self) -> CH2OM3_W {
        CH2OM3_W { w: self }
    }
}
