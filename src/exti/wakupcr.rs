#[doc = "Reader of register WAKUPCR"]
pub type R = crate::R<u32, super::WAKUPCR>;
#[doc = "Writer for register WAKUPCR"]
pub type W = crate::W<u32, super::WAKUPCR>;
#[doc = "Register WAKUPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKUPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0WEN`"]
pub type EXTI0WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0WEN`"]
pub struct EXTI0WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EXTI1WEN`"]
pub type EXTI1WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1WEN`"]
pub struct EXTI1WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EXTI2WEN`"]
pub type EXTI2WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2WEN`"]
pub struct EXTI2WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EXTI3WEN`"]
pub type EXTI3WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3WEN`"]
pub struct EXTI3WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EXTI4WEN`"]
pub type EXTI4WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4WEN`"]
pub struct EXTI4WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI5WEN`"]
pub type EXTI5WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5WEN`"]
pub struct EXTI5WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EXTI6WEN`"]
pub type EXTI6WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6WEN`"]
pub struct EXTI6WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EXTI7WEN`"]
pub type EXTI7WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7WEN`"]
pub struct EXTI7WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EXTI8WEN`"]
pub type EXTI8WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8WEN`"]
pub struct EXTI8WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI9WEN`"]
pub type EXTI9WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9WEN`"]
pub struct EXTI9WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EXTI10WEN`"]
pub type EXTI10WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10WEN`"]
pub struct EXTI10WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EXTI11WEN`"]
pub type EXTI11WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11WEN`"]
pub struct EXTI11WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EXTI12WEN`"]
pub type EXTI12WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12WEN`"]
pub struct EXTI12WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI13WEN`"]
pub type EXTI13WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13WEN`"]
pub struct EXTI13WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EXTI14WEN`"]
pub type EXTI14WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14WEN`"]
pub struct EXTI14WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EXTI15WEN`"]
pub type EXTI15WEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15WEN`"]
pub struct EXTI15WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15WEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `EVWUPIEN`"]
pub type EVWUPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVWUPIEN`"]
pub struct EVWUPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVWUPIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    pub fn exti0wen(&self) -> EXTI0WEN_R {
        EXTI0WEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    pub fn exti1wen(&self) -> EXTI1WEN_R {
        EXTI1WEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    pub fn exti2wen(&self) -> EXTI2WEN_R {
        EXTI2WEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    pub fn exti3wen(&self) -> EXTI3WEN_R {
        EXTI3WEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    pub fn exti4wen(&self) -> EXTI4WEN_R {
        EXTI4WEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    pub fn exti5wen(&self) -> EXTI5WEN_R {
        EXTI5WEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    pub fn exti6wen(&self) -> EXTI6WEN_R {
        EXTI6WEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    pub fn exti7wen(&self) -> EXTI7WEN_R {
        EXTI7WEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    pub fn exti8wen(&self) -> EXTI8WEN_R {
        EXTI8WEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    pub fn exti9wen(&self) -> EXTI9WEN_R {
        EXTI9WEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    pub fn exti10wen(&self) -> EXTI10WEN_R {
        EXTI10WEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    pub fn exti11wen(&self) -> EXTI11WEN_R {
        EXTI11WEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    pub fn exti12wen(&self) -> EXTI12WEN_R {
        EXTI12WEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    pub fn exti13wen(&self) -> EXTI13WEN_R {
        EXTI13WEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    pub fn exti14wen(&self) -> EXTI14WEN_R {
        EXTI14WEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    pub fn exti15wen(&self) -> EXTI15WEN_R {
        EXTI15WEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    pub fn evwupien(&self) -> EVWUPIEN_R {
        EVWUPIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WEN"]
    #[inline(always)]
    pub fn exti0wen(&mut self) -> EXTI0WEN_W {
        EXTI0WEN_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1WEN"]
    #[inline(always)]
    pub fn exti1wen(&mut self) -> EXTI1WEN_W {
        EXTI1WEN_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2WEN"]
    #[inline(always)]
    pub fn exti2wen(&mut self) -> EXTI2WEN_W {
        EXTI2WEN_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3WEN"]
    #[inline(always)]
    pub fn exti3wen(&mut self) -> EXTI3WEN_W {
        EXTI3WEN_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4WEN"]
    #[inline(always)]
    pub fn exti4wen(&mut self) -> EXTI4WEN_W {
        EXTI4WEN_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5WEN"]
    #[inline(always)]
    pub fn exti5wen(&mut self) -> EXTI5WEN_W {
        EXTI5WEN_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6WEN"]
    #[inline(always)]
    pub fn exti6wen(&mut self) -> EXTI6WEN_W {
        EXTI6WEN_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7WEN"]
    #[inline(always)]
    pub fn exti7wen(&mut self) -> EXTI7WEN_W {
        EXTI7WEN_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8WEN"]
    #[inline(always)]
    pub fn exti8wen(&mut self) -> EXTI8WEN_W {
        EXTI8WEN_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9WEN"]
    #[inline(always)]
    pub fn exti9wen(&mut self) -> EXTI9WEN_W {
        EXTI9WEN_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10WEN"]
    #[inline(always)]
    pub fn exti10wen(&mut self) -> EXTI10WEN_W {
        EXTI10WEN_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11WEN"]
    #[inline(always)]
    pub fn exti11wen(&mut self) -> EXTI11WEN_W {
        EXTI11WEN_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12WEN"]
    #[inline(always)]
    pub fn exti12wen(&mut self) -> EXTI12WEN_W {
        EXTI12WEN_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13WEN"]
    #[inline(always)]
    pub fn exti13wen(&mut self) -> EXTI13WEN_W {
        EXTI13WEN_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14WEN"]
    #[inline(always)]
    pub fn exti14wen(&mut self) -> EXTI14WEN_W {
        EXTI14WEN_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15WEN"]
    #[inline(always)]
    pub fn exti15wen(&mut self) -> EXTI15WEN_W {
        EXTI15WEN_W { w: self }
    }
    #[doc = "Bit 31 - EVWUPIEN"]
    #[inline(always)]
    pub fn evwupien(&mut self) -> EVWUPIEN_W {
        EVWUPIEN_W { w: self }
    }
}
