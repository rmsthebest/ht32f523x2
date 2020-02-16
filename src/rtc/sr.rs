#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSECFLAG`"]
pub type CSECFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSECFLAG`"]
pub struct CSECFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CSECFLAG_W<'a> {
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
#[doc = "Reader of field `CMFLAG`"]
pub type CMFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMFLAG`"]
pub struct CMFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMFLAG_W<'a> {
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
#[doc = "Reader of field `OVFLAG`"]
pub type OVFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVFLAG`"]
pub struct OVFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFLAG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    pub fn csecflag(&self) -> CSECFLAG_R {
        CSECFLAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    pub fn cmflag(&self) -> CMFLAG_R {
        CMFLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    pub fn ovflag(&self) -> OVFLAG_R {
        OVFLAG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    pub fn csecflag(&mut self) -> CSECFLAG_W {
        CSECFLAG_W { w: self }
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    pub fn cmflag(&mut self) -> CMFLAG_W {
        CMFLAG_W { w: self }
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    pub fn ovflag(&mut self) -> OVFLAG_W {
        OVFLAG_W { w: self }
    }
}
