#[doc = "Reader of register GPTM_INTSR"]
pub type R = crate::R<u32, super::GPTM_INTSR>;
#[doc = "Writer for register GPTM_INTSR"]
pub type W = crate::W<u32, super::GPTM_INTSR>;
#[doc = "Register GPTM_INTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_INTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CCIF`"]
pub type CH0CCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CCIF`"]
pub struct CH0CCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCIF_W<'a> {
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
#[doc = "Reader of field `CH1CCIF`"]
pub type CH1CCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CCIF`"]
pub struct CH1CCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCIF_W<'a> {
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
#[doc = "Reader of field `CH2CCIF`"]
pub type CH2CCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2CCIF`"]
pub struct CH2CCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCIF_W<'a> {
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
#[doc = "Reader of field `CH3CCIF`"]
pub type CH3CCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3CCIF`"]
pub struct CH3CCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCIF_W<'a> {
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
#[doc = "Reader of field `CH0OCF`"]
pub type CH0OCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0OCF`"]
pub struct CH0OCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OCF_W<'a> {
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
#[doc = "Reader of field `CH1OCF`"]
pub type CH1OCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OCF`"]
pub struct CH1OCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OCF_W<'a> {
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
#[doc = "Reader of field `CH2OCF`"]
pub type CH2OCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OCF`"]
pub struct CH2OCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OCF_W<'a> {
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
#[doc = "Reader of field `CH3OCF`"]
pub type CH3OCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3OCF`"]
pub struct CH3OCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OCF_W<'a> {
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
#[doc = "Reader of field `UEVIF`"]
pub type UEVIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEVIF`"]
pub struct UEVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UEVIF_W<'a> {
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
#[doc = "Reader of field `TEVIF`"]
pub type TEVIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEVIF`"]
pub struct TEVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEVIF_W<'a> {
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
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&self) -> CH0CCIF_R {
        CH0CCIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    pub fn ch1ccif(&self) -> CH1CCIF_R {
        CH1CCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    pub fn ch2ccif(&self) -> CH2CCIF_R {
        CH2CCIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    pub fn ch3ccif(&self) -> CH3CCIF_R {
        CH3CCIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&self) -> CH0OCF_R {
        CH0OCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    pub fn ch1ocf(&self) -> CH1OCF_R {
        CH1OCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    pub fn ch2ocf(&self) -> CH2OCF_R {
        CH2OCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    pub fn ch3ocf(&self) -> CH3OCF_R {
        CH3OCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&self) -> UEVIF_R {
        UEVIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TEVIF_R {
        TEVIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&mut self) -> CH0CCIF_W {
        CH0CCIF_W { w: self }
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    pub fn ch1ccif(&mut self) -> CH1CCIF_W {
        CH1CCIF_W { w: self }
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    pub fn ch2ccif(&mut self) -> CH2CCIF_W {
        CH2CCIF_W { w: self }
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    pub fn ch3ccif(&mut self) -> CH3CCIF_W {
        CH3CCIF_W { w: self }
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&mut self) -> CH0OCF_W {
        CH0OCF_W { w: self }
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    pub fn ch1ocf(&mut self) -> CH1OCF_W {
        CH1OCF_W { w: self }
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    pub fn ch2ocf(&mut self) -> CH2OCF_W {
        CH2OCF_W { w: self }
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    pub fn ch3ocf(&mut self) -> CH3OCF_W {
        CH3OCF_W { w: self }
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&mut self) -> UEVIF_W {
        UEVIF_W { w: self }
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&mut self) -> TEVIF_W {
        TEVIF_W { w: self }
    }
}
