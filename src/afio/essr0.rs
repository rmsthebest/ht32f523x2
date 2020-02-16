#[doc = "Reader of register ESSR0"]
pub type R = crate::R<u32, super::ESSR0>;
#[doc = "Writer for register ESSR0"]
pub type W = crate::W<u32, super::ESSR0>;
#[doc = "Register ESSR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0PIN`"]
pub type EXTI0PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI0PIN`"]
pub struct EXTI0PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EXTI1PIN`"]
pub type EXTI1PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI1PIN`"]
pub struct EXTI1PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI2PIN`"]
pub type EXTI2PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI2PIN`"]
pub struct EXTI2PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI3PIN`"]
pub type EXTI3PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI3PIN`"]
pub struct EXTI3PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI4PIN`"]
pub type EXTI4PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI4PIN`"]
pub struct EXTI4PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTI5PIN`"]
pub type EXTI5PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI5PIN`"]
pub struct EXTI5PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `EXTI6PIN`"]
pub type EXTI6PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI6PIN`"]
pub struct EXTI6PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EXTI7PIN`"]
pub type EXTI7PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI7PIN`"]
pub struct EXTI7PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    pub fn exti0pin(&self) -> EXTI0PIN_R {
        EXTI0PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    pub fn exti1pin(&self) -> EXTI1PIN_R {
        EXTI1PIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    pub fn exti2pin(&self) -> EXTI2PIN_R {
        EXTI2PIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    pub fn exti3pin(&self) -> EXTI3PIN_R {
        EXTI3PIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    pub fn exti4pin(&self) -> EXTI4PIN_R {
        EXTI4PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    pub fn exti5pin(&self) -> EXTI5PIN_R {
        EXTI5PIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    pub fn exti6pin(&self) -> EXTI6PIN_R {
        EXTI6PIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    pub fn exti7pin(&self) -> EXTI7PIN_R {
        EXTI7PIN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline(always)]
    pub fn exti0pin(&mut self) -> EXTI0PIN_W {
        EXTI0PIN_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline(always)]
    pub fn exti1pin(&mut self) -> EXTI1PIN_W {
        EXTI1PIN_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline(always)]
    pub fn exti2pin(&mut self) -> EXTI2PIN_W {
        EXTI2PIN_W { w: self }
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline(always)]
    pub fn exti3pin(&mut self) -> EXTI3PIN_W {
        EXTI3PIN_W { w: self }
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline(always)]
    pub fn exti4pin(&mut self) -> EXTI4PIN_W {
        EXTI4PIN_W { w: self }
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline(always)]
    pub fn exti5pin(&mut self) -> EXTI5PIN_W {
        EXTI5PIN_W { w: self }
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline(always)]
    pub fn exti6pin(&mut self) -> EXTI6PIN_W {
        EXTI6PIN_W { w: self }
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline(always)]
    pub fn exti7pin(&mut self) -> EXTI7PIN_W {
        EXTI7PIN_W { w: self }
    }
}
