#[doc = "Reader of register GCIR"]
pub type R = crate::R<u32, super::GCIR>;
#[doc = "Writer for register GCIR"]
pub type W = crate::W<u32, super::GCIR>;
#[doc = "Register GCIR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKSF`"]
pub type CKSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSF`"]
pub struct CKSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSF_W<'a> {
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
#[doc = "Reader of field `PLLRDYF`"]
pub type PLLRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLRDYF`"]
pub struct PLLRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYF_W<'a> {
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
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSERDYF`"]
pub struct HSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYF_W<'a> {
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
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIRDYF`"]
pub struct HSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYF_W<'a> {
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
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSERDYF`"]
pub struct LSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYF_W<'a> {
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
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSIRDYF`"]
pub struct LSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYF_W<'a> {
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
#[doc = "Reader of field `CKSIE`"]
pub type CKSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSIE`"]
pub struct CKSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSIE_W<'a> {
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
#[doc = "Reader of field `PLLRDYIE`"]
pub type PLLRDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLRDYIE`"]
pub struct PLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSERDYIE`"]
pub type HSERDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSERDYIE`"]
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HSIRDYIE`"]
pub type HSIRDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIRDYIE`"]
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LSERDYIE`"]
pub type LSERDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSERDYIE`"]
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LSIRDYIE`"]
pub type LSIRDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSIRDYIE`"]
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    pub fn cksf(&self) -> CKSF_R {
        CKSF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    pub fn cksie(&self) -> CKSIE_R {
        CKSIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    pub fn cksf(&mut self) -> CKSF_W {
        CKSF_W { w: self }
    }
    #[doc = "Bit 2 - PLLRDYF"]
    #[inline(always)]
    pub fn pllrdyf(&mut self) -> PLLRDYF_W {
        PLLRDYF_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HSERDYF_W {
        HSERDYF_W { w: self }
    }
    #[doc = "Bit 4 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W {
        HSIRDYF_W { w: self }
    }
    #[doc = "Bit 5 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LSERDYF_W {
        LSERDYF_W { w: self }
    }
    #[doc = "Bit 6 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W {
        LSIRDYF_W { w: self }
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    pub fn cksie(&mut self) -> CKSIE_W {
        CKSIE_W { w: self }
    }
    #[doc = "Bit 18 - PLLRDYIE"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W { w: self }
    }
    #[doc = "Bit 19 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 20 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 21 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 22 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
}
