#[doc = "Reader of register CHBRKCFR"]
pub type R = crate::R<u32, super::CHBRKCFR>;
#[doc = "Writer for register CHBRKCFR"]
pub type W = crate::W<u32, super::CHBRKCFR>;
#[doc = "Register CHBRKCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHBRKCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0OIS`"]
pub type CH0OIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OIS`"]
pub struct CH0OIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OIS_W<'a> {
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
#[doc = "Reader of field `CH0OISN`"]
pub type CH0OISN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OISN`"]
pub struct CH0OISN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OISN_W<'a> {
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
#[doc = "Reader of field `CH1OIS`"]
pub type CH1OIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OIS`"]
pub struct CH1OIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OIS_W<'a> {
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
#[doc = "Reader of field `CH1OISN`"]
pub type CH1OISN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OISN`"]
pub struct CH1OISN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OISN_W<'a> {
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
#[doc = "Reader of field `CH2OIS`"]
pub type CH2OIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OIS`"]
pub struct CH2OIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OIS_W<'a> {
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
#[doc = "Reader of field `CH2OISN`"]
pub type CH2OISN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OISN`"]
pub struct CH2OISN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OISN_W<'a> {
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
#[doc = "Reader of field `CH3OIS`"]
pub type CH3OIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3OIS`"]
pub struct CH3OIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OIS_W<'a> {
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
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    pub fn ch0ois(&self) -> CH0OIS_R {
        CH0OIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    pub fn ch0oisn(&self) -> CH0OISN_R {
        CH0OISN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    pub fn ch1ois(&self) -> CH1OIS_R {
        CH1OIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    pub fn ch1oisn(&self) -> CH1OISN_R {
        CH1OISN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    pub fn ch2ois(&self) -> CH2OIS_R {
        CH2OIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    pub fn ch2oisn(&self) -> CH2OISN_R {
        CH2OISN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    pub fn ch3ois(&self) -> CH3OIS_R {
        CH3OIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    pub fn ch0ois(&mut self) -> CH0OIS_W {
        CH0OIS_W { w: self }
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    pub fn ch0oisn(&mut self) -> CH0OISN_W {
        CH0OISN_W { w: self }
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    pub fn ch1ois(&mut self) -> CH1OIS_W {
        CH1OIS_W { w: self }
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    pub fn ch1oisn(&mut self) -> CH1OISN_W {
        CH1OISN_W { w: self }
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    pub fn ch2ois(&mut self) -> CH2OIS_W {
        CH2OIS_W { w: self }
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    pub fn ch2oisn(&mut self) -> CH2OISN_W {
        CH2OISN_W { w: self }
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    pub fn ch3ois(&mut self) -> CH3OIS_W {
        CH3OIS_W { w: self }
    }
}
