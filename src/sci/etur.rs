#[doc = "Reader of register ETUR"]
pub type R = crate::R<u32, super::ETUR>;
#[doc = "Writer for register ETUR"]
pub type W = crate::W<u32, super::ETUR>;
#[doc = "Register ETUR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETU`"]
pub type ETU_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ETU`"]
pub struct ETU_W<'a> {
    w: &'a mut W,
}
impl<'a> ETU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    pub fn etu(&self) -> ETU_R {
        ETU_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    pub fn etu(&mut self) -> ETU_W {
        ETU_W { w: self }
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
}
