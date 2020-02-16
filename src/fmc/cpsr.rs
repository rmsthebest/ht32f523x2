#[doc = "Reader of register CPSR"]
pub type R = crate::R<u32, super::CPSR>;
#[doc = "Writer for register CPSR"]
pub type W = crate::W<u32, super::CPSR>;
#[doc = "Register CPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPSB`"]
pub type CPSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPSB`"]
pub struct CPSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSB_W<'a> {
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
#[doc = "Reader of field `OBPSB`"]
pub type OBPSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBPSB`"]
pub struct OBPSB_W<'a> {
    w: &'a mut W,
}
impl<'a> OBPSB_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    pub fn cpsb(&self) -> CPSB_R {
        CPSB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    pub fn obpsb(&self) -> OBPSB_R {
        OBPSB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    pub fn cpsb(&mut self) -> CPSB_W {
        CPSB_W { w: self }
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    pub fn obpsb(&mut self) -> OBPSB_W {
        OBPSB_W { w: self }
    }
}
