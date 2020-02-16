#[doc = "Reader of register DR5"]
pub type R = crate::R<u32, super::DR5>;
#[doc = "Writer for register DR5"]
pub type W = crate::W<u32, super::DR5>;
#[doc = "Register DR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD5`"]
pub type ADD5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD5`"]
pub struct ADD5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD5`"]
pub type ADVLD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD5`"]
pub struct ADVLD5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD5_W<'a> {
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
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    pub fn add5(&self) -> ADD5_R {
        ADD5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    pub fn advld5(&self) -> ADVLD5_R {
        ADVLD5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    pub fn add5(&mut self) -> ADD5_W {
        ADD5_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    pub fn advld5(&mut self) -> ADVLD5_W {
        ADVLD5_W { w: self }
    }
}
