#[doc = "Reader of register DR7"]
pub type R = crate::R<u32, super::DR7>;
#[doc = "Writer for register DR7"]
pub type W = crate::W<u32, super::DR7>;
#[doc = "Register DR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD7`"]
pub type ADD7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD7`"]
pub struct ADD7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD7`"]
pub type ADVLD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD7`"]
pub struct ADVLD7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD7_W<'a> {
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
    #[doc = "Bits 0:15 - ADD7"]
    #[inline(always)]
    pub fn add7(&self) -> ADD7_R {
        ADD7_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD7"]
    #[inline(always)]
    pub fn advld7(&self) -> ADVLD7_R {
        ADVLD7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD7"]
    #[inline(always)]
    pub fn add7(&mut self) -> ADD7_W {
        ADD7_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD7"]
    #[inline(always)]
    pub fn advld7(&mut self) -> ADVLD7_W {
        ADVLD7_W { w: self }
    }
}
