#[doc = "Reader of register ESSR1"]
pub type R = crate::R<u32, super::ESSR1>;
#[doc = "Writer for register ESSR1"]
pub type W = crate::W<u32, super::ESSR1>;
#[doc = "Register ESSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI8PIN`"]
pub type EXTI8PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI8PIN`"]
pub struct EXTI8PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EXTI9PIN`"]
pub type EXTI9PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI9PIN`"]
pub struct EXTI9PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI10PIN`"]
pub type EXTI10PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI10PIN`"]
pub struct EXTI10PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI11PIN`"]
pub type EXTI11PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI11PIN`"]
pub struct EXTI11PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI12PIN`"]
pub type EXTI12PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI12PIN`"]
pub struct EXTI12PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTI13PIN`"]
pub type EXTI13PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI13PIN`"]
pub struct EXTI13PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `EXTI14PIN`"]
pub type EXTI14PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI14PIN`"]
pub struct EXTI14PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EXTI15PIN`"]
pub type EXTI15PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI15PIN`"]
pub struct EXTI15PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    pub fn exti8pin(&self) -> EXTI8PIN_R {
        EXTI8PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    pub fn exti9pin(&self) -> EXTI9PIN_R {
        EXTI9PIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    pub fn exti10pin(&self) -> EXTI10PIN_R {
        EXTI10PIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    pub fn exti11pin(&self) -> EXTI11PIN_R {
        EXTI11PIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    pub fn exti12pin(&self) -> EXTI12PIN_R {
        EXTI12PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    pub fn exti13pin(&self) -> EXTI13PIN_R {
        EXTI13PIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    pub fn exti14pin(&self) -> EXTI14PIN_R {
        EXTI14PIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    pub fn exti15pin(&self) -> EXTI15PIN_R {
        EXTI15PIN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline(always)]
    pub fn exti8pin(&mut self) -> EXTI8PIN_W {
        EXTI8PIN_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline(always)]
    pub fn exti9pin(&mut self) -> EXTI9PIN_W {
        EXTI9PIN_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline(always)]
    pub fn exti10pin(&mut self) -> EXTI10PIN_W {
        EXTI10PIN_W { w: self }
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline(always)]
    pub fn exti11pin(&mut self) -> EXTI11PIN_W {
        EXTI11PIN_W { w: self }
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline(always)]
    pub fn exti12pin(&mut self) -> EXTI12PIN_W {
        EXTI12PIN_W { w: self }
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline(always)]
    pub fn exti13pin(&mut self) -> EXTI13PIN_W {
        EXTI13PIN_W { w: self }
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline(always)]
    pub fn exti14pin(&mut self) -> EXTI14PIN_W {
        EXTI14PIN_W { w: self }
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline(always)]
    pub fn exti15pin(&mut self) -> EXTI15PIN_W {
        EXTI15PIN_W { w: self }
    }
}
