#[doc = "Reader of register EP4ISR"]
pub type R = crate::R<u32, super::EP4ISR>;
#[doc = "Writer for register EP4ISR"]
pub type W = crate::W<u32, super::EP4ISR>;
#[doc = "Register EP4ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP4ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTRXIF`"]
pub type OTRXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTRXIF`"]
pub struct OTRXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRXIF_W<'a> {
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
#[doc = "Reader of field `ODRXIF`"]
pub type ODRXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODRXIF`"]
pub struct ODRXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ODRXIF_W<'a> {
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
#[doc = "Reader of field `ODOVIF`"]
pub type ODOVIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODOVIF`"]
pub struct ODOVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ODOVIF_W<'a> {
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
#[doc = "Reader of field `ITRXIF`"]
pub type ITRXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITRXIF`"]
pub struct ITRXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRXIF_W<'a> {
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
#[doc = "Reader of field `IDTXIF`"]
pub type IDTXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDTXIF`"]
pub struct IDTXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTXIF_W<'a> {
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
#[doc = "Reader of field `NAKIF`"]
pub type NAKIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKIF`"]
pub struct NAKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKIF_W<'a> {
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
#[doc = "Reader of field `STLIF`"]
pub type STLIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STLIF`"]
pub struct STLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STLIF_W<'a> {
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
#[doc = "Reader of field `UERIF`"]
pub type UERIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UERIF`"]
pub struct UERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UERIF_W<'a> {
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
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    pub fn otrxif(&self) -> OTRXIF_R {
        OTRXIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    pub fn odrxif(&self) -> ODRXIF_R {
        ODRXIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    pub fn odovif(&self) -> ODOVIF_R {
        ODOVIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    pub fn itrxif(&self) -> ITRXIF_R {
        ITRXIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    pub fn idtxif(&self) -> IDTXIF_R {
        IDTXIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    pub fn nakif(&self) -> NAKIF_R {
        NAKIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    pub fn stlif(&self) -> STLIF_R {
        STLIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    pub fn uerif(&self) -> UERIF_R {
        UERIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    pub fn otrxif(&mut self) -> OTRXIF_W {
        OTRXIF_W { w: self }
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    pub fn odrxif(&mut self) -> ODRXIF_W {
        ODRXIF_W { w: self }
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    pub fn odovif(&mut self) -> ODOVIF_W {
        ODOVIF_W { w: self }
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    pub fn itrxif(&mut self) -> ITRXIF_W {
        ITRXIF_W { w: self }
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    pub fn idtxif(&mut self) -> IDTXIF_W {
        IDTXIF_W { w: self }
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    pub fn nakif(&mut self) -> NAKIF_W {
        NAKIF_W { w: self }
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    pub fn stlif(&mut self) -> STLIF_W {
        STLIF_W { w: self }
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    pub fn uerif(&mut self) -> UERIF_W {
        UERIF_W { w: self }
    }
}
