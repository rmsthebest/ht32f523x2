#[doc = "Reader of register CH3ICFR"]
pub type R = crate::R<u32, super::CH3ICFR>;
#[doc = "Writer for register CH3ICFR"]
pub type W = crate::W<u32, super::CH3ICFR>;
#[doc = "Register CH3ICFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3ICFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI3F`"]
pub type TI3F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI3F`"]
pub struct TI3F_W<'a> {
    w: &'a mut W,
}
impl<'a> TI3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH3CCS`"]
pub type CH3CCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3CCS`"]
pub struct CH3CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH3PSC`"]
pub type CH3PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3PSC`"]
pub struct CH3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    pub fn ti3f(&self) -> TI3F_R {
        TI3F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    pub fn ch3ccs(&self) -> CH3CCS_R {
        CH3CCS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    pub fn ch3psc(&self) -> CH3PSC_R {
        CH3PSC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI3F"]
    #[inline(always)]
    pub fn ti3f(&mut self) -> TI3F_W {
        TI3F_W { w: self }
    }
    #[doc = "Bits 16:17 - CH3CCS"]
    #[inline(always)]
    pub fn ch3ccs(&mut self) -> CH3CCS_W {
        CH3CCS_W { w: self }
    }
    #[doc = "Bits 18:19 - CH3PSC"]
    #[inline(always)]
    pub fn ch3psc(&mut self) -> CH3PSC_W {
        CH3PSC_W { w: self }
    }
}
