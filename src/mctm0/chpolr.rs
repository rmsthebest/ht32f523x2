#[doc = "Reader of register CHPOLR"]
pub type R = crate::R<u32, super::CHPOLR>;
#[doc = "Writer for register CHPOLR"]
pub type W = crate::W<u32, super::CHPOLR>;
#[doc = "Register CHPOLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPOLR {
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
#[doc = "Reader of field `CH0NP`"]
pub type CH0NP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NP`"]
pub struct CH0NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NP_W<'a> {
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
#[doc = "Reader of field `CH1NP`"]
pub type CH1NP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1NP`"]
pub struct CH1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1NP_W<'a> {
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
#[doc = "Reader of field `CH2NP`"]
pub type CH2NP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2NP`"]
pub struct CH2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2NP_W<'a> {
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
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    pub fn ch2np(&self) -> CH2NP_R {
        CH2NP_R::new(((self.bits >> 5) & 0x01) != 0)
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
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    pub fn ch0np(&mut self) -> CH0NP_W {
        CH0NP_W { w: self }
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&mut self) -> CH1P_W {
        CH1P_W { w: self }
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    pub fn ch1np(&mut self) -> CH1NP_W {
        CH1NP_W { w: self }
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&mut self) -> CH2P_W {
        CH2P_W { w: self }
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    pub fn ch2np(&mut self) -> CH2NP_W {
        CH2NP_W { w: self }
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&mut self) -> CH3P_W {
        CH3P_W { w: self }
    }
}
