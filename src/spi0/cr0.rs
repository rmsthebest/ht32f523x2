#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPIEN`"]
pub type SPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIEN`"]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
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
#[doc = "Reader of field `TXDMAE`"]
pub type TXDMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMAE`"]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
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
#[doc = "Reader of field `RXDMAE`"]
pub type RXDMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAE`"]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
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
#[doc = "Reader of field `SELOEN`"]
pub type SELOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELOEN`"]
pub struct SELOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SELOEN_W<'a> {
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
#[doc = "Reader of field `SSELC`"]
pub type SSELC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSELC`"]
pub struct SSELC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSELC_W<'a> {
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
#[doc = "Reader of field `DUALEN`"]
pub type DUALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALEN`"]
pub struct DUALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALEN_W<'a> {
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
#[doc = "Reader of field `GUADTEN`"]
pub type GUADTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GUADTEN`"]
pub struct GUADTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GUADTEN_W<'a> {
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
#[doc = "Reader of field `GUADT`"]
pub type GUADT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GUADT`"]
pub struct GUADT_W<'a> {
    w: &'a mut W,
}
impl<'a> GUADT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SELHT`"]
pub type SELHT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELHT`"]
pub struct SELHT_W<'a> {
    w: &'a mut W,
}
impl<'a> SELHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&self) -> SELOEN_R {
        SELOEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&self) -> SSELC_R {
        SSELC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&self) -> DUALEN_R {
        DUALEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    pub fn guadten(&self) -> GUADTEN_R {
        GUADTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    pub fn guadt(&self) -> GUADT_R {
        GUADT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    pub fn selht(&self) -> SELHT_R {
        SELHT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&mut self) -> SELOEN_W {
        SELOEN_W { w: self }
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&mut self) -> SSELC_W {
        SSELC_W { w: self }
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&mut self) -> DUALEN_W {
        DUALEN_W { w: self }
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    pub fn guadten(&mut self) -> GUADTEN_W {
        GUADTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    pub fn guadt(&mut self) -> GUADT_W {
        GUADT_W { w: self }
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    pub fn selht(&mut self) -> SELHT_W {
        SELHT_W { w: self }
    }
}
