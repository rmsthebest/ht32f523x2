#[doc = "Reader of register DR3"]
pub type R = crate::R<u32, super::DR3>;
#[doc = "Writer for register DR3"]
pub type W = crate::W<u32, super::DR3>;
#[doc = "Register DR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD3`"]
pub type ADD3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD3`"]
pub struct ADD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD3`"]
pub type ADVLD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD3`"]
pub struct ADVLD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD3_W<'a> {
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
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    pub fn add3(&self) -> ADD3_R {
        ADD3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    pub fn advld3(&self) -> ADVLD3_R {
        ADVLD3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    pub fn add3(&mut self) -> ADD3_W {
        ADD3_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    pub fn advld3(&mut self) -> ADVLD3_W {
        ADVLD3_W { w: self }
    }
}
