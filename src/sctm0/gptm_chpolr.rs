#[doc = "Reader of register GPTM_CHPOLR"]
pub type R = crate::R<u32, super::GPTM_CHPOLR>;
#[doc = "Writer for register GPTM_CHPOLR"]
pub type W = crate::W<u32, super::GPTM_CHPOLR>;
#[doc = "Register GPTM_CHPOLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CHPOLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0P`"]
pub type CH0P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0P`"]
pub struct CH0P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0P_W<'a> {
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
#[doc = "Reader of field `CH1P`"]
pub type CH1P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1P`"]
pub struct CH1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1P_W<'a> {
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
#[doc = "Reader of field `CH2P`"]
pub type CH2P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2P`"]
pub struct CH2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2P_W<'a> {
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
#[doc = "Reader of field `CH3P`"]
pub type CH3P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3P`"]
pub struct CH3P_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3P_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    pub fn ch0p(&mut self) -> CH0P_W {
        CH0P_W { w: self }
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> CH1P_W {
        CH1P_W { w: self }
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> CH2P_W {
        CH2P_W { w: self }
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> CH3P_W {
        CH3P_W { w: self }
    }
}
