#[doc = "Reader of register EP2IER"]
pub type R = crate::R<u32, super::EP2IER>;
#[doc = "Writer for register EP2IER"]
pub type W = crate::W<u32, super::EP2IER>;
#[doc = "Register EP2IER `reset()`'s with value 0"]
impl crate::ResetValue for super::EP2IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTRXIE`"]
pub type OTRXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTRXIE`"]
pub struct OTRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRXIE_W<'a> {
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
#[doc = "Reader of field `ODRXIE`"]
pub type ODRXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODRXIE`"]
pub struct ODRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODRXIE_W<'a> {
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
#[doc = "Reader of field `ODOVIE`"]
pub type ODOVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODOVIE`"]
pub struct ODOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODOVIE_W<'a> {
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
#[doc = "Reader of field `ITRXIE`"]
pub type ITRXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITRXIE`"]
pub struct ITRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRXIE_W<'a> {
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
#[doc = "Reader of field `IDTXIE`"]
pub type IDTXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDTXIE`"]
pub struct IDTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTXIE_W<'a> {
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
#[doc = "Reader of field `NAKIE`"]
pub type NAKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKIE`"]
pub struct NAKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKIE_W<'a> {
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
#[doc = "Reader of field `STLIE`"]
pub type STLIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STLIE`"]
pub struct STLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STLIE_W<'a> {
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
#[doc = "Reader of field `UERIE`"]
pub type UERIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UERIE`"]
pub struct UERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UERIE_W<'a> {
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
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    pub fn otrxie(&self) -> OTRXIE_R {
        OTRXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    pub fn odrxie(&self) -> ODRXIE_R {
        ODRXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    pub fn odovie(&self) -> ODOVIE_R {
        ODOVIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    pub fn itrxie(&self) -> ITRXIE_R {
        ITRXIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    pub fn idtxie(&self) -> IDTXIE_R {
        IDTXIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    pub fn stlie(&self) -> STLIE_R {
        STLIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    pub fn uerie(&self) -> UERIE_R {
        UERIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    pub fn otrxie(&mut self) -> OTRXIE_W {
        OTRXIE_W { w: self }
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    pub fn odrxie(&mut self) -> ODRXIE_W {
        ODRXIE_W { w: self }
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    pub fn odovie(&mut self) -> ODOVIE_W {
        ODOVIE_W { w: self }
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    pub fn itrxie(&mut self) -> ITRXIE_W {
        ITRXIE_W { w: self }
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    pub fn idtxie(&mut self) -> IDTXIE_W {
        IDTXIE_W { w: self }
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    pub fn nakie(&mut self) -> NAKIE_W {
        NAKIE_W { w: self }
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    pub fn stlie(&mut self) -> STLIE_W {
        STLIE_W { w: self }
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    pub fn uerie(&mut self) -> UERIE_W {
        UERIE_W { w: self }
    }
}
