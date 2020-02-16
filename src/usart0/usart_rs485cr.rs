#[doc = "Reader of register USART_RS485CR"]
pub type R = crate::R<u32, super::USART_RS485CR>;
#[doc = "Writer for register USART_RS485CR"]
pub type W = crate::W<u32, super::USART_RS485CR>;
#[doc = "Register USART_RS485CR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_RS485CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXENP`"]
pub type TXENP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENP`"]
pub struct TXENP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENP_W<'a> {
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
#[doc = "Reader of field `RSNMM`"]
pub type RSNMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSNMM`"]
pub struct RSNMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSNMM_W<'a> {
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
#[doc = "Reader of field `RSAAD`"]
pub type RSAAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSAAD`"]
pub struct RSAAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSAAD_W<'a> {
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
#[doc = "Reader of field `ADDMATCH`"]
pub type ADDMATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDMATCH`"]
pub struct ADDMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    pub fn txenp(&self) -> TXENP_R {
        TXENP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    pub fn rsnmm(&self) -> RSNMM_R {
        RSNMM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    pub fn rsaad(&self) -> RSAAD_R {
        RSAAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    pub fn addmatch(&self) -> ADDMATCH_R {
        ADDMATCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    pub fn txenp(&mut self) -> TXENP_W {
        TXENP_W { w: self }
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    pub fn rsnmm(&mut self) -> RSNMM_W {
        RSNMM_W { w: self }
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    pub fn rsaad(&mut self) -> RSAAD_W {
        RSAAD_W { w: self }
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    pub fn addmatch(&mut self) -> ADDMATCH_W {
        ADDMATCH_W { w: self }
    }
}
