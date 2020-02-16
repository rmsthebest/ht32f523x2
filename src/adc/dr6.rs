#[doc = "Reader of register DR6"]
pub type R = crate::R<u32, super::DR6>;
#[doc = "Writer for register DR6"]
pub type W = crate::W<u32, super::DR6>;
#[doc = "Register DR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD6`"]
pub type ADD6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD6`"]
pub struct ADD6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD6`"]
pub type ADVLD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD6`"]
pub struct ADVLD6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD6_W<'a> {
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
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    pub fn add6(&self) -> ADD6_R {
        ADD6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    pub fn advld6(&self) -> ADVLD6_R {
        ADVLD6_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD6"]
    #[inline(always)]
    pub fn add6(&mut self) -> ADD6_W {
        ADD6_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD6"]
    #[inline(always)]
    pub fn advld6(&mut self) -> ADVLD6_W {
        ADVLD6_W { w: self }
    }
}
