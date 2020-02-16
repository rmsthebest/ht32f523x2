#[doc = "Reader of register LST0"]
pub type R = crate::R<u32, super::LST0>;
#[doc = "Writer for register LST0"]
pub type W = crate::W<u32, super::LST0>;
#[doc = "Register LST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LST0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSEQ0`"]
pub type ADSEQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ0`"]
pub struct ADSEQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ1`"]
pub type ADSEQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ1`"]
pub struct ADSEQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ2`"]
pub type ADSEQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ2`"]
pub struct ADSEQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ3`"]
pub type ADSEQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ3`"]
pub struct ADSEQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    pub fn adseq0(&self) -> ADSEQ0_R {
        ADSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    pub fn adseq1(&self) -> ADSEQ1_R {
        ADSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    pub fn adseq2(&self) -> ADSEQ2_R {
        ADSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    pub fn adseq3(&self) -> ADSEQ3_R {
        ADSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    pub fn adseq0(&mut self) -> ADSEQ0_W {
        ADSEQ0_W { w: self }
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    pub fn adseq1(&mut self) -> ADSEQ1_W {
        ADSEQ1_W { w: self }
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    pub fn adseq2(&mut self) -> ADSEQ2_W {
        ADSEQ2_W { w: self }
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    pub fn adseq3(&mut self) -> ADSEQ3_W {
        ADSEQ3_W { w: self }
    }
}
