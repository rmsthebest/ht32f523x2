#[doc = "Reader of register USART_SYNCR"]
pub type R = crate::R<u32, super::USART_SYNCR>;
#[doc = "Writer for register USART_SYNCR"]
pub type W = crate::W<u32, super::USART_SYNCR>;
#[doc = "Register USART_SYNCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_SYNCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
#[doc = "Reader of field `CPS`"]
pub type CPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPS`"]
pub struct CPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CPO`"]
pub type CPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPO`"]
pub struct CPO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    pub fn cpo(&self) -> CPO_R {
        CPO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    pub fn cps(&mut self) -> CPS_W {
        CPS_W { w: self }
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    pub fn cpo(&mut self) -> CPO_W {
        CPO_W { w: self }
    }
}
