#[doc = "Reader of register DR0"]
pub type R = crate::R<u32, super::DR0>;
#[doc = "Writer for register DR0"]
pub type W = crate::W<u32, super::DR0>;
#[doc = "Register DR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD0`"]
pub type ADD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD0`"]
pub struct ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD0`"]
pub type ADVLD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD0`"]
pub struct ADVLD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD0_W<'a> {
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
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    pub fn advld0(&self) -> ADVLD0_R {
        ADVLD0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD0"]
    #[inline(always)]
    pub fn add0(&mut self) -> ADD0_W {
        ADD0_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD0"]
    #[inline(always)]
    pub fn advld0(&mut self) -> ADVLD0_W {
        ADVLD0_W { w: self }
    }
}
