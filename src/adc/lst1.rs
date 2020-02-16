#[doc = "Reader of register LST1"]
pub type R = crate::R<u32, super::LST1>;
#[doc = "Writer for register LST1"]
pub type W = crate::W<u32, super::LST1>;
#[doc = "Register LST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LST1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSEQ4`"]
pub type ADSEQ4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ4`"]
pub struct ADSEQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ5`"]
pub type ADSEQ5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ5`"]
pub struct ADSEQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ6`"]
pub type ADSEQ6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ6`"]
pub struct ADSEQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ7`"]
pub type ADSEQ7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ7`"]
pub struct ADSEQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    pub fn adseq4(&self) -> ADSEQ4_R {
        ADSEQ4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    pub fn adseq5(&self) -> ADSEQ5_R {
        ADSEQ5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    pub fn adseq6(&self) -> ADSEQ6_R {
        ADSEQ6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    pub fn adseq7(&self) -> ADSEQ7_R {
        ADSEQ7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    pub fn adseq4(&mut self) -> ADSEQ4_W {
        ADSEQ4_W { w: self }
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    pub fn adseq5(&mut self) -> ADSEQ5_W {
        ADSEQ5_W { w: self }
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    pub fn adseq6(&mut self) -> ADSEQ6_W {
        ADSEQ6_W { w: self }
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    pub fn adseq7(&mut self) -> ADSEQ7_W {
        ADSEQ7_W { w: self }
    }
}
