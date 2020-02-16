#[doc = "Reader of register IER0"]
pub type R = crate::R<u32, super::IER0>;
#[doc = "Writer for register IER0"]
pub type W = crate::W<u32, super::IER0>;
#[doc = "Register IER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPFIEN`"]
pub type CMPFIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPFIEN`"]
pub struct CMPFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFIEN_W<'a> {
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
#[doc = "Reader of field `CMPRIEN`"]
pub type CMPRIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPRIEN`"]
pub struct CMPRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPRIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    pub fn cmpfien(&self) -> CMPFIEN_R {
        CMPFIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    pub fn cmprien(&self) -> CMPRIEN_R {
        CMPRIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    pub fn cmpfien(&mut self) -> CMPFIEN_W {
        CMPFIEN_W { w: self }
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    pub fn cmprien(&mut self) -> CMPRIEN_W {
        CMPRIEN_W { w: self }
    }
}
