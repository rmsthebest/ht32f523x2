#[doc = "Reader of register CHCTR"]
pub type R = crate::R<u32, super::CHCTR>;
#[doc = "Writer for register CHCTR"]
pub type W = crate::W<u32, super::CHCTR>;
#[doc = "Register CHCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0E`"]
pub type CH0E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0E`"]
pub struct CH0E_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0E_W<'a> {
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
#[doc = "Reader of field `CH0NE`"]
pub type CH0NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NE`"]
pub struct CH0NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NE_W<'a> {
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
#[doc = "Reader of field `CH1E`"]
pub type CH1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1E`"]
pub struct CH1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1E_W<'a> {
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
#[doc = "Reader of field `CH1NE`"]
pub type CH1NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1NE`"]
pub struct CH1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1NE_W<'a> {
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
#[doc = "Reader of field `CH2E`"]
pub type CH2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2E`"]
pub struct CH2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2E_W<'a> {
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
#[doc = "Reader of field `CH2NE`"]
pub type CH2NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2NE`"]
pub struct CH2NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2NE_W<'a> {
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
#[doc = "Reader of field `CH3E`"]
pub type CH3E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3E`"]
pub struct CH3E_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3E_W<'a> {
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
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&self) -> CH0E_R {
        CH0E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    pub fn ch0ne(&self) -> CH0NE_R {
        CH0NE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&self) -> CH1E_R {
        CH1E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    pub fn ch1ne(&self) -> CH1NE_R {
        CH1NE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&self) -> CH2E_R {
        CH2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    pub fn ch2ne(&self) -> CH2NE_R {
        CH2NE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&self) -> CH3E_R {
        CH3E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0E"]
    #[inline(always)]
    pub fn ch0e(&mut self) -> CH0E_W {
        CH0E_W { w: self }
    }
    #[doc = "Bit 1 - CH0NE"]
    #[inline(always)]
    pub fn ch0ne(&mut self) -> CH0NE_W {
        CH0NE_W { w: self }
    }
    #[doc = "Bit 2 - CH1E"]
    #[inline(always)]
    pub fn ch1e(&mut self) -> CH1E_W {
        CH1E_W { w: self }
    }
    #[doc = "Bit 3 - CH1NE"]
    #[inline(always)]
    pub fn ch1ne(&mut self) -> CH1NE_W {
        CH1NE_W { w: self }
    }
    #[doc = "Bit 4 - CH2E"]
    #[inline(always)]
    pub fn ch2e(&mut self) -> CH2E_W {
        CH2E_W { w: self }
    }
    #[doc = "Bit 5 - CH2NE"]
    #[inline(always)]
    pub fn ch2ne(&mut self) -> CH2NE_W {
        CH2NE_W { w: self }
    }
    #[doc = "Bit 6 - CH3E"]
    #[inline(always)]
    pub fn ch3e(&mut self) -> CH3E_W {
        CH3E_W { w: self }
    }
}
