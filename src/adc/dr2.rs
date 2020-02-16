#[doc = "Reader of register DR2"]
pub type R = crate::R<u32, super::DR2>;
#[doc = "Writer for register DR2"]
pub type W = crate::W<u32, super::DR2>;
#[doc = "Register DR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD2`"]
pub type ADD2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD2`"]
pub struct ADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD2`"]
pub type ADVLD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD2`"]
pub struct ADVLD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD2_W<'a> {
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
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    pub fn advld2(&self) -> ADVLD2_R {
        ADVLD2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    pub fn advld2(&mut self) -> ADVLD2_W {
        ADVLD2_W { w: self }
    }
}
