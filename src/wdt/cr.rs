#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTRS`"]
pub type WDTRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRS`"]
pub struct WDTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RSKEY`"]
pub type RSKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSKEY`"]
pub struct RSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RSKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDTRS"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WDTRS_R {
        WDTRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - RSKEY"]
    #[inline(always)]
    pub fn rskey(&self) -> RSKEY_R {
        RSKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - WDTRS"]
    #[inline(always)]
    pub fn wdtrs(&mut self) -> WDTRS_W {
        WDTRS_W { w: self }
    }
    #[doc = "Bits 16:31 - RSKEY"]
    #[inline(always)]
    pub fn rskey(&mut self) -> RSKEY_W {
        RSKEY_W { w: self }
    }
}
