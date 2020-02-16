#[doc = "Reader of register HSICR"]
pub type R = crate::R<u32, super::HSICR>;
#[doc = "Writer for register HSICR"]
pub type W = crate::W<u32, super::HSICR>;
#[doc = "Register HSICR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIMEN`"]
pub type TRIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIMEN`"]
pub struct TRIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMEN_W<'a> {
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
#[doc = "Reader of field `ATCEN`"]
pub type ATCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATCEN`"]
pub struct ATCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCEN_W<'a> {
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
#[doc = "Reader of field `TMSEL`"]
pub type TMSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMSEL`"]
pub struct TMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSEL_W<'a> {
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
#[doc = "Reader of field `REFCLKSEL`"]
pub type REFCLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFCLKSEL`"]
pub struct REFCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLKSEL_W<'a> {
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
#[doc = "Reader of field `FLOCK`"]
pub type FLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLOCK`"]
pub struct FLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLOCK_W<'a> {
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
#[doc = "Reader of field `HSIFINE`"]
pub type HSIFINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSIFINE`"]
pub struct HSIFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIFINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSICOARSE`"]
pub type HSICOARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSICOARSE`"]
pub struct HSICOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    pub fn trimen(&self) -> TRIMEN_R {
        TRIMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    pub fn atcen(&self) -> ATCEN_R {
        ATCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    pub fn tmsel(&self) -> TMSEL_R {
        TMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    pub fn refclksel(&self) -> REFCLKSEL_R {
        REFCLKSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    pub fn flock(&self) -> FLOCK_R {
        FLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    pub fn hsifine(&self) -> HSIFINE_R {
        HSIFINE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    pub fn hsicoarse(&self) -> HSICOARSE_R {
        HSICOARSE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRIMEN"]
    #[inline(always)]
    pub fn trimen(&mut self) -> TRIMEN_W {
        TRIMEN_W { w: self }
    }
    #[doc = "Bit 1 - ATCEN"]
    #[inline(always)]
    pub fn atcen(&mut self) -> ATCEN_W {
        ATCEN_W { w: self }
    }
    #[doc = "Bit 4 - TMSEL"]
    #[inline(always)]
    pub fn tmsel(&mut self) -> TMSEL_W {
        TMSEL_W { w: self }
    }
    #[doc = "Bit 5 - REFCLKSEL"]
    #[inline(always)]
    pub fn refclksel(&mut self) -> REFCLKSEL_W {
        REFCLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - FLOCK"]
    #[inline(always)]
    pub fn flock(&mut self) -> FLOCK_W {
        FLOCK_W { w: self }
    }
    #[doc = "Bits 16:23 - HSIFINE"]
    #[inline(always)]
    pub fn hsifine(&mut self) -> HSIFINE_W {
        HSIFINE_W { w: self }
    }
    #[doc = "Bits 24:28 - HSICOARSE"]
    #[inline(always)]
    pub fn hsicoarse(&mut self) -> HSICOARSE_W {
        HSICOARSE_W { w: self }
    }
}
