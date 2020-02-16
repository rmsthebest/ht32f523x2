#[doc = "Reader of register CH3OCFR"]
pub type R = crate::R<u32, super::CH3OCFR>;
#[doc = "Writer for register CH3OCFR"]
pub type W = crate::W<u32, super::CH3OCFR>;
#[doc = "Register CH3OCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3OCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3OM`"]
pub type CH3OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3OM`"]
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CH3PRE`"]
pub type CH3PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3PRE`"]
pub struct CH3PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PRE_W<'a> {
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
#[doc = "Reader of field `CH3IMAE`"]
pub type CH3IMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3IMAE`"]
pub struct CH3IMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3IMAE_W<'a> {
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
#[doc = "Reader of field `CH3OM3`"]
pub type CH3OM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3OM3`"]
pub struct CH3OM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM3_W<'a> {
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
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    pub fn ch3pre(&self) -> CH3PRE_R {
        CH3PRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    pub fn ch3imae(&self) -> CH3IMAE_R {
        CH3IMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    pub fn ch3om3(&self) -> CH3OM3_R {
        CH3OM3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    pub fn ch3pre(&mut self) -> CH3PRE_W {
        CH3PRE_W { w: self }
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    pub fn ch3imae(&mut self) -> CH3IMAE_W {
        CH3IMAE_W { w: self }
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    pub fn ch3om3(&mut self) -> CH3OM3_W {
        CH3OM3_W { w: self }
    }
}
