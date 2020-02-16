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
#[doc = "Reader of field `TXFTLIEN`"]
pub type TXFTLIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFTLIEN`"]
pub struct TXFTLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTLIEN_W<'a> {
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
#[doc = "Reader of field `TXUDIEN`"]
pub type TXUDIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUDIEN`"]
pub struct TXUDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDIEN_W<'a> {
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
#[doc = "Reader of field `TXOVIEN`"]
pub type TXOVIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOVIEN`"]
pub struct TXOVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVIEN_W<'a> {
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
#[doc = "Reader of field `RXFTLIEN`"]
pub type RXFTLIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFTLIEN`"]
pub struct RXFTLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTLIEN_W<'a> {
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
#[doc = "Reader of field `RXUDIEN`"]
pub type RXUDIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDIEN`"]
pub struct RXUDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDIEN_W<'a> {
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
#[doc = "Reader of field `RXOVIEN`"]
pub type RXOVIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVIEN`"]
pub struct RXOVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVIEN_W<'a> {
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
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    pub fn txftlien(&self) -> TXFTLIEN_R {
        TXFTLIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    pub fn txudien(&self) -> TXUDIEN_R {
        TXUDIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    pub fn txovien(&self) -> TXOVIEN_R {
        TXOVIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    pub fn rxftlien(&self) -> RXFTLIEN_R {
        RXFTLIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    pub fn rxudien(&self) -> RXUDIEN_R {
        RXUDIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    pub fn rxovien(&self) -> RXOVIEN_R {
        RXOVIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTLIEN"]
    #[inline(always)]
    pub fn txftlien(&mut self) -> TXFTLIEN_W {
        TXFTLIEN_W { w: self }
    }
    #[doc = "Bit 1 - TXUDIEN"]
    #[inline(always)]
    pub fn txudien(&mut self) -> TXUDIEN_W {
        TXUDIEN_W { w: self }
    }
    #[doc = "Bit 2 - TXOVIEN"]
    #[inline(always)]
    pub fn txovien(&mut self) -> TXOVIEN_W {
        TXOVIEN_W { w: self }
    }
    #[doc = "Bit 4 - RXFTLIEN"]
    #[inline(always)]
    pub fn rxftlien(&mut self) -> RXFTLIEN_W {
        RXFTLIEN_W { w: self }
    }
    #[doc = "Bit 5 - RXUDIEN"]
    #[inline(always)]
    pub fn rxudien(&mut self) -> RXUDIEN_W {
        RXUDIEN_W { w: self }
    }
    #[doc = "Bit 6 - RXOVIEN"]
    #[inline(always)]
    pub fn rxovien(&mut self) -> RXOVIEN_W {
        RXOVIEN_W { w: self }
    }
}
