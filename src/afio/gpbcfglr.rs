#[doc = "Reader of register GPBCFGLR"]
pub type R = crate::R<u32, super::GPBCFGLR>;
#[doc = "Writer for register GPBCFGLR"]
pub type W = crate::W<u32, super::GPBCFGLR>;
#[doc = "Register GPBCFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPBCFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFG0`"]
pub type CFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG0`"]
pub struct CFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG1`"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFG2`"]
pub type CFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG2`"]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFG3`"]
pub type CFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG3`"]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CFG4`"]
pub type CFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG4`"]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFG5`"]
pub type CFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG5`"]
pub struct CFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CFG6`"]
pub type CFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG6`"]
pub struct CFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CFG7`"]
pub type CFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG7`"]
pub struct CFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    pub fn cfg0(&mut self) -> CFG0_W {
        CFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W { w: self }
    }
}
