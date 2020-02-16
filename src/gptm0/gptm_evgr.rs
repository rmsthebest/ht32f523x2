#[doc = "Reader of register GPTM_EVGR"]
pub type R = crate::R<u32, super::GPTM_EVGR>;
#[doc = "Writer for register GPTM_EVGR"]
pub type W = crate::W<u32, super::GPTM_EVGR>;
#[doc = "Register GPTM_EVGR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_EVGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CCG`"]
pub type CH0CCG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CCG`"]
pub struct CH0CCG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCG_W<'a> {
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
#[doc = "Reader of field `CH1CCG`"]
pub type CH1CCG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CCG`"]
pub struct CH1CCG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCG_W<'a> {
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
#[doc = "Reader of field `CH2CCG`"]
pub type CH2CCG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2CCG`"]
pub struct CH2CCG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCG_W<'a> {
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
#[doc = "Reader of field `CH3CCG`"]
pub type CH3CCG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3CCG`"]
pub struct CH3CCG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCG_W<'a> {
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
#[doc = "Reader of field `UEVG`"]
pub type UEVG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEVG`"]
pub struct UEVG_W<'a> {
    w: &'a mut W,
}
impl<'a> UEVG_W<'a> {
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
#[doc = "Reader of field `TEVG`"]
pub type TEVG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEVG`"]
pub struct TEVG_W<'a> {
    w: &'a mut W,
}
impl<'a> TEVG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    pub fn ch0ccg(&self) -> CH0CCG_R {
        CH0CCG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    pub fn ch1ccg(&self) -> CH1CCG_R {
        CH1CCG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    pub fn ch2ccg(&self) -> CH2CCG_R {
        CH2CCG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    pub fn ch3ccg(&self) -> CH3CCG_R {
        CH3CCG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&self) -> UEVG_R {
        UEVG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&self) -> TEVG_R {
        TEVG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    pub fn ch0ccg(&mut self) -> CH0CCG_W {
        CH0CCG_W { w: self }
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    pub fn ch1ccg(&mut self) -> CH1CCG_W {
        CH1CCG_W { w: self }
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    pub fn ch2ccg(&mut self) -> CH2CCG_W {
        CH2CCG_W { w: self }
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    pub fn ch3ccg(&mut self) -> CH3CCG_W {
        CH3CCG_W { w: self }
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&mut self) -> UEVG_W {
        UEVG_W { w: self }
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&mut self) -> TEVG_W {
        TEVG_W { w: self }
    }
}
