#[doc = "Reader of register TAR"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Writer for register TAR"]
pub type W = crate::W<u32, super::TAR>;
#[doc = "Register TAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TAR`"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `RWD`"]
pub type RWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWD`"]
pub struct RWD_W<'a> {
    w: &'a mut W,
}
impl<'a> RWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    pub fn rwd(&self) -> RWD_R {
        RWD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    pub fn rwd(&mut self) -> RWD_W {
        RWD_W { w: self }
    }
}
