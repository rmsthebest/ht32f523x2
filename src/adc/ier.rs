#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADIES`"]
pub type ADIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIES`"]
pub struct ADIES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIES_W<'a> {
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
#[doc = "Reader of field `ADIEG`"]
pub type ADIEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIEG`"]
pub struct ADIEG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIEG_W<'a> {
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
#[doc = "Reader of field `ADIEC`"]
pub type ADIEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIEC`"]
pub struct ADIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIEC_W<'a> {
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
#[doc = "Reader of field `ADIEL`"]
pub type ADIEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIEL`"]
pub struct ADIEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADIEU`"]
pub type ADIEU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIEU`"]
pub struct ADIEU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIEU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ADIEO`"]
pub type ADIEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIEO`"]
pub struct ADIEO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    pub fn adies(&self) -> ADIES_R {
        ADIES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    pub fn adieg(&self) -> ADIEG_R {
        ADIEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    pub fn adiec(&self) -> ADIEC_R {
        ADIEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    pub fn adiel(&self) -> ADIEL_R {
        ADIEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    pub fn adieu(&self) -> ADIEU_R {
        ADIEU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    pub fn adieo(&self) -> ADIEO_R {
        ADIEO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    pub fn adies(&mut self) -> ADIES_W {
        ADIES_W { w: self }
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    pub fn adieg(&mut self) -> ADIEG_W {
        ADIEG_W { w: self }
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    pub fn adiec(&mut self) -> ADIEC_W {
        ADIEC_W { w: self }
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    pub fn adiel(&mut self) -> ADIEL_W {
        ADIEL_W { w: self }
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    pub fn adieu(&mut self) -> ADIEU_W {
        ADIEU_W { w: self }
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    pub fn adieo(&mut self) -> ADIEO_W {
        ADIEO_W { w: self }
    }
}
