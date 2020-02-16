#[doc = "Reader of register DR4"]
pub type R = crate::R<u32, super::DR4>;
#[doc = "Writer for register DR4"]
pub type W = crate::W<u32, super::DR4>;
#[doc = "Register DR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD4`"]
pub type ADD4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD4`"]
pub struct ADD4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD4`"]
pub type ADVLD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD4`"]
pub struct ADVLD4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD4_W<'a> {
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
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    pub fn add4(&self) -> ADD4_R {
        ADD4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    pub fn advld4(&self) -> ADVLD4_R {
        ADVLD4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    pub fn add4(&mut self) -> ADD4_W {
        ADD4_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    pub fn advld4(&mut self) -> ADVLD4_W {
        ADVLD4_W { w: self }
    }
}
