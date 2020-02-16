#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTUF`"]
pub type WDTUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTUF`"]
pub struct WDTUF_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTUF_W<'a> {
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
#[doc = "Reader of field `WDTERR`"]
pub type WDTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTERR`"]
pub struct WDTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTERR_W<'a> {
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
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    pub fn wdtuf(&self) -> WDTUF_R {
        WDTUF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&self) -> WDTERR_R {
        WDTERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    pub fn wdtuf(&mut self) -> WDTUF_W {
        WDTUF_W { w: self }
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&mut self) -> WDTERR_W {
        WDTERR_W { w: self }
    }
}
