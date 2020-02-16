#[doc = "Reader of register DR1"]
pub type R = crate::R<u32, super::DR1>;
#[doc = "Writer for register DR1"]
pub type W = crate::W<u32, super::DR1>;
#[doc = "Register DR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD1`"]
pub type ADD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD1`"]
pub struct ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD1`"]
pub type ADVLD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD1`"]
pub struct ADVLD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    pub fn advld1(&self) -> ADVLD1_R {
        ADVLD1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    pub fn add1(&mut self) -> ADD1_W {
        ADD1_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    pub fn advld1(&mut self) -> ADVLD1_W {
        ADVLD1_W { w: self }
    }
}
