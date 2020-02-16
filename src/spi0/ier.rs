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
#[doc = "Reader of field `TXBEIEN`"]
pub type TXBEIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBEIEN`"]
pub struct TXBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBEIEN_W<'a> {
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
#[doc = "Reader of field `TXEIEN`"]
pub type TXEIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEIEN`"]
pub struct TXEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIEN_W<'a> {
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
#[doc = "Reader of field `RXBNEIEN`"]
pub type RXBNEIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBNEIEN`"]
pub struct RXBNEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBNEIEN_W<'a> {
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
#[doc = "Reader of field `WCIEN`"]
pub type WCIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCIEN`"]
pub struct WCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WCIEN_W<'a> {
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
#[doc = "Reader of field `ROIEN`"]
pub type ROIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROIEN`"]
pub struct ROIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIEN_W<'a> {
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
#[doc = "Reader of field `MFIEN`"]
pub type MFIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MFIEN`"]
pub struct MFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MFIEN_W<'a> {
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
#[doc = "Reader of field `SAIEN`"]
pub type SAIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAIEN`"]
pub struct SAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIEN_W<'a> {
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
#[doc = "Reader of field `TOIEN`"]
pub type TOIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOIEN`"]
pub struct TOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    pub fn txbeien(&self) -> TXBEIEN_R {
        TXBEIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    pub fn txeien(&self) -> TXEIEN_R {
        TXEIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    pub fn rxbneien(&self) -> RXBNEIEN_R {
        RXBNEIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    pub fn wcien(&self) -> WCIEN_R {
        WCIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    pub fn mfien(&self) -> MFIEN_R {
        MFIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    pub fn txbeien(&mut self) -> TXBEIEN_W {
        TXBEIEN_W { w: self }
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    pub fn txeien(&mut self) -> TXEIEN_W {
        TXEIEN_W { w: self }
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    pub fn rxbneien(&mut self) -> RXBNEIEN_W {
        RXBNEIEN_W { w: self }
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    pub fn wcien(&mut self) -> WCIEN_W {
        WCIEN_W { w: self }
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    pub fn roien(&mut self) -> ROIEN_W {
        ROIEN_W { w: self }
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    pub fn mfien(&mut self) -> MFIEN_W {
        MFIEN_W { w: self }
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    pub fn saien(&mut self) -> SAIEN_W {
        SAIEN_W { w: self }
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    pub fn toien(&mut self) -> TOIEN_W {
        TOIEN_W { w: self }
    }
}
