#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRNUM`"]
pub type FRNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRNUM`"]
pub struct FRNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `SOFLCK`"]
pub type SOFLCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFLCK`"]
pub struct SOFLCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFLCK_W<'a> {
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
#[doc = "Reader of field `LSOF`"]
pub type LSOF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSOF`"]
pub struct LSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    pub fn soflck(&self) -> SOFLCK_R {
        SOFLCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&mut self) -> FRNUM_W {
        FRNUM_W { w: self }
    }
    #[doc = "Bit 16 - SOFLCK"]
    #[inline(always)]
    pub fn soflck(&mut self) -> SOFLCK_W {
        SOFLCK_W { w: self }
    }
    #[doc = "Bits 17:18 - LSOF"]
    #[inline(always)]
    pub fn lsof(&mut self) -> LSOF_W {
        LSOF_W { w: self }
    }
}
