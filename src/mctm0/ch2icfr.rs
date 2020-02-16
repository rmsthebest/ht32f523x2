#[doc = "Reader of register CH2ICFR"]
pub type R = crate::R<u32, super::CH2ICFR>;
#[doc = "Writer for register CH2ICFR"]
pub type W = crate::W<u32, super::CH2ICFR>;
#[doc = "Register CH2ICFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2ICFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI2F`"]
pub type TI2F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI2F`"]
pub struct TI2F_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH2CCS`"]
pub type CH2CCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2CCS`"]
pub struct CH2CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH2PSC`"]
pub type CH2PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2PSC`"]
pub struct CH2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    pub fn ti2f(&self) -> TI2F_R {
        TI2F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    pub fn ch2ccs(&self) -> CH2CCS_R {
        CH2CCS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    pub fn ch2psc(&self) -> CH2PSC_R {
        CH2PSC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI2F"]
    #[inline(always)]
    pub fn ti2f(&mut self) -> TI2F_W {
        TI2F_W { w: self }
    }
    #[doc = "Bits 16:17 - CH2CCS"]
    #[inline(always)]
    pub fn ch2ccs(&mut self) -> CH2CCS_W {
        CH2CCS_W { w: self }
    }
    #[doc = "Bits 18:19 - CH2PSC"]
    #[inline(always)]
    pub fn ch2psc(&mut self) -> CH2PSC_W {
        CH2PSC_W { w: self }
    }
}
