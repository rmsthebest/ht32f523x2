#[doc = "Reader of register MR1"]
pub type R = crate::R<u32, super::MR1>;
#[doc = "Writer for register MR1"]
pub type W = crate::W<u32, super::MR1>;
#[doc = "Register MR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTD`"]
pub type WDTD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDTD`"]
pub struct WDTD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `WPSC`"]
pub type WPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WPSC`"]
pub struct WPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    pub fn wdtd(&self) -> WDTD_R {
        WDTD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    pub fn wpsc(&self) -> WPSC_R {
        WPSC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    pub fn wdtd(&mut self) -> WDTD_W {
        WDTD_W { w: self }
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    pub fn wpsc(&mut self) -> WPSC_W {
        WPSC_W { w: self }
    }
}
