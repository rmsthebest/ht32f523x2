#[doc = "Reader of register PLLCR"]
pub type R = crate::R<u32, super::PLLCR>;
#[doc = "Writer for register PLLCR"]
pub type W = crate::W<u32, super::PLLCR>;
#[doc = "Register PLLCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLBPS`"]
pub type PLLBPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLBPS`"]
pub struct PLLBPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBPS_W<'a> {
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
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    pub fn pllbps(&self) -> PLLBPS_R {
        PLLBPS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - PLLBPS"]
    #[inline(always)]
    pub fn pllbps(&mut self) -> PLLBPS_W {
        PLLBPS_W { w: self }
    }
}
