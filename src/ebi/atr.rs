#[doc = "Reader of register ATR"]
pub type R = crate::R<u32, super::ATR>;
#[doc = "Writer for register ATR"]
pub type W = crate::W<u32, super::ATR>;
#[doc = "Register ATR `reset()`'s with value 0"]
impl crate::ResetValue for super::ATR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRSETUP`"]
pub type ADDRSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRSETUP`"]
pub struct ADDRSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDRHOLD`"]
pub type ADDRHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRHOLD`"]
pub struct ADDRHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W {
        ADDRSETUP_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    pub fn addrhold(&mut self) -> ADDRHOLD_W {
        ADDRHOLD_W { w: self }
    }
}
