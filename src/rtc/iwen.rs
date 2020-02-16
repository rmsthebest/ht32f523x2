#[doc = "Reader of register IWEN"]
pub type R = crate::R<u32, super::IWEN>;
#[doc = "Writer for register IWEN"]
pub type W = crate::W<u32, super::IWEN>;
#[doc = "Register IWEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSECIEN`"]
pub type CSECIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSECIEN`"]
pub struct CSECIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSECIEN_W<'a> {
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
#[doc = "Reader of field `CMIEN`"]
pub type CMIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMIEN`"]
pub struct CMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMIEN_W<'a> {
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
#[doc = "Reader of field `OVIEN`"]
pub type OVIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVIEN`"]
pub struct OVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIEN_W<'a> {
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
#[doc = "Reader of field `CSECWEN`"]
pub type CSECWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSECWEN`"]
pub struct CSECWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSECWEN_W<'a> {
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
#[doc = "Reader of field `CMWEN`"]
pub type CMWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMWEN`"]
pub struct CMWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `OVWEN`"]
pub type OVWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVWEN`"]
pub struct OVWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVWEN_W<'a> {
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
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    pub fn csecien(&self) -> CSECIEN_R {
        CSECIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    pub fn cmien(&self) -> CMIEN_R {
        CMIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    pub fn ovien(&self) -> OVIEN_R {
        OVIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    pub fn csecwen(&self) -> CSECWEN_R {
        CSECWEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    pub fn cmwen(&self) -> CMWEN_R {
        CMWEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    pub fn ovwen(&self) -> OVWEN_R {
        OVWEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    pub fn csecien(&mut self) -> CSECIEN_W {
        CSECIEN_W { w: self }
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    pub fn cmien(&mut self) -> CMIEN_W {
        CMIEN_W { w: self }
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    pub fn ovien(&mut self) -> OVIEN_W {
        OVIEN_W { w: self }
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    pub fn csecwen(&mut self) -> CSECWEN_W {
        CSECWEN_W { w: self }
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    pub fn cmwen(&mut self) -> CMWEN_W {
        CMWEN_W { w: self }
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    pub fn ovwen(&mut self) -> OVWEN_W {
        OVWEN_W { w: self }
    }
}
