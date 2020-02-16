#[doc = "Reader of register GPCCFGHR"]
pub type R = crate::R<u32, super::GPCCFGHR>;
#[doc = "Writer for register GPCCFGHR"]
pub type W = crate::W<u32, super::GPCCFGHR>;
#[doc = "Register GPCCFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCCFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFG8`"]
pub type CFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG8`"]
pub struct CFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CFG9`"]
pub type CFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG9`"]
pub struct CFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFG10`"]
pub type CFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG10`"]
pub struct CFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFG11`"]
pub type CFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG11`"]
pub struct CFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CFG12`"]
pub type CFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG12`"]
pub struct CFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFG13`"]
pub type CFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG13`"]
pub struct CFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CFG14`"]
pub type CFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG14`"]
pub struct CFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CFG15`"]
pub type CFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG15`"]
pub struct CFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    pub fn cfg9(&self) -> CFG9_R {
        CFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    pub fn cfg10(&self) -> CFG10_R {
        CFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    pub fn cfg11(&self) -> CFG11_R {
        CFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    pub fn cfg12(&self) -> CFG12_R {
        CFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    pub fn cfg13(&self) -> CFG13_R {
        CFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    pub fn cfg14(&self) -> CFG14_R {
        CFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    pub fn cfg15(&self) -> CFG15_R {
        CFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&mut self) -> CFG8_W {
        CFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    pub fn cfg9(&mut self) -> CFG9_W {
        CFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    pub fn cfg10(&mut self) -> CFG10_W {
        CFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    pub fn cfg11(&mut self) -> CFG11_W {
        CFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    pub fn cfg12(&mut self) -> CFG12_W {
        CFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    pub fn cfg13(&mut self) -> CFG13_W {
        CFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    pub fn cfg14(&mut self) -> CFG14_W {
        CFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    pub fn cfg15(&mut self) -> CFG15_W {
        CFG15_W { w: self }
    }
}
