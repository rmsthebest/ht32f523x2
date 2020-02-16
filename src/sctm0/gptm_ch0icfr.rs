#[doc = "Reader of register GPTM_CH0ICFR"]
pub type R = crate::R<u32, super::GPTM_CH0ICFR>;
#[doc = "Writer for register GPTM_CH0ICFR"]
pub type W = crate::W<u32, super::GPTM_CH0ICFR>;
#[doc = "Register GPTM_CH0ICFR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CH0ICFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI0F`"]
pub type TI0F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI0F`"]
pub struct TI0F_W<'a> {
    w: &'a mut W,
}
impl<'a> TI0F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH0CCS`"]
pub type CH0CCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0CCS`"]
pub struct CH0CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH0PSC`"]
pub type CH0PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0PSC`"]
pub struct CH0PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `TI0SRC`"]
pub type TI0SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI0SRC`"]
pub struct TI0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TI0SRC_W<'a> {
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
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    pub fn ti0f(&self) -> TI0F_R {
        TI0F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    pub fn ch0ccs(&self) -> CH0CCS_R {
        CH0CCS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    pub fn ch0psc(&self) -> CH0PSC_R {
        CH0PSC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    pub fn ti0src(&self) -> TI0SRC_R {
        TI0SRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    pub fn ti0f(&mut self) -> TI0F_W {
        TI0F_W { w: self }
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    pub fn ch0ccs(&mut self) -> CH0CCS_W {
        CH0CCS_W { w: self }
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    pub fn ch0psc(&mut self) -> CH0PSC_W {
        CH0PSC_W { w: self }
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    pub fn ti0src(&mut self) -> TI0SRC_W {
        TI0SRC_W { w: self }
    }
}
