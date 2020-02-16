#[doc = "Reader of register RTR"]
pub type R = crate::R<u32, super::RTR>;
#[doc = "Writer for register RTR"]
pub type W = crate::W<u32, super::RTR>;
#[doc = "Register RTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDSETUP`"]
pub type RDSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSETUP`"]
pub struct RDSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RDSTRB`"]
pub type RDSTRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSTRB`"]
pub struct RDSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSTRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RDHOLD`"]
pub type RDHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDHOLD`"]
pub struct RDHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RDHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RDSETUP_W {
        RDSETUP_W { w: self }
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RDSTRB_W {
        RDSTRB_W { w: self }
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W {
        RDHOLD_W { w: self }
    }
}
