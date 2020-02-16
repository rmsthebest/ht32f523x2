#[doc = "Reader of register GPTM_CH1ICFR"]
pub type R = crate::R<u32, super::GPTM_CH1ICFR>;
#[doc = "Writer for register GPTM_CH1ICFR"]
pub type W = crate::W<u32, super::GPTM_CH1ICFR>;
#[doc = "Register GPTM_CH1ICFR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CH1ICFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI1F`"]
pub type TI1F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI1F`"]
pub struct TI1F_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH1CCS`"]
pub type CH1CCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1CCS`"]
pub struct CH1CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH1PSC`"]
pub type CH1PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1PSC`"]
pub struct CH1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    pub fn ti1f(&self) -> TI1F_R {
        TI1F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    pub fn ch1ccs(&self) -> CH1CCS_R {
        CH1CCS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    pub fn ch1psc(&self) -> CH1PSC_R {
        CH1PSC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1F"]
    #[inline(always)]
    pub fn ti1f(&mut self) -> TI1F_W {
        TI1F_W { w: self }
    }
    #[doc = "Bits 16:17 - CH1CCS"]
    #[inline(always)]
    pub fn ch1ccs(&mut self) -> CH1CCS_W {
        CH1CCS_W { w: self }
    }
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    pub fn ch1psc(&mut self) -> CH1PSC_W {
        CH1PSC_W { w: self }
    }
}
