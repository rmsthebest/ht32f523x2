#[doc = "Reader of register WAKUPFLG"]
pub type R = crate::R<u32, super::WAKUPFLG>;
#[doc = "Writer for register WAKUPFLG"]
pub type W = crate::W<u32, super::WAKUPFLG>;
#[doc = "Register WAKUPFLG `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKUPFLG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0WFL`"]
pub type EXTI0WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0WFL`"]
pub struct EXTI0WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0WFL_W<'a> {
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
#[doc = "Reader of field `EXTI1WFL`"]
pub type EXTI1WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1WFL`"]
pub struct EXTI1WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1WFL_W<'a> {
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
#[doc = "Reader of field `EXTI2WFL`"]
pub type EXTI2WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2WFL`"]
pub struct EXTI2WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2WFL_W<'a> {
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
#[doc = "Reader of field `EXTI3WFL`"]
pub type EXTI3WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3WFL`"]
pub struct EXTI3WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3WFL_W<'a> {
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
#[doc = "Reader of field `EXTI4WFL`"]
pub type EXTI4WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4WFL`"]
pub struct EXTI4WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4WFL_W<'a> {
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
#[doc = "Reader of field `EXTI5WFL`"]
pub type EXTI5WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5WFL`"]
pub struct EXTI5WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5WFL_W<'a> {
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
#[doc = "Reader of field `EXTI6WFL`"]
pub type EXTI6WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6WFL`"]
pub struct EXTI6WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6WFL_W<'a> {
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
#[doc = "Reader of field `EXTI7WFL`"]
pub type EXTI7WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7WFL`"]
pub struct EXTI7WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7WFL_W<'a> {
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
#[doc = "Reader of field `EXTI8WFL`"]
pub type EXTI8WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8WFL`"]
pub struct EXTI8WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8WFL_W<'a> {
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
#[doc = "Reader of field `EXTI9WFL`"]
pub type EXTI9WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9WFL`"]
pub struct EXTI9WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9WFL_W<'a> {
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
#[doc = "Reader of field `EXTI10WFL`"]
pub type EXTI10WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10WFL`"]
pub struct EXTI10WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10WFL_W<'a> {
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
#[doc = "Reader of field `EXTI11WFL`"]
pub type EXTI11WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11WFL`"]
pub struct EXTI11WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11WFL_W<'a> {
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
#[doc = "Reader of field `EXTI12WFL`"]
pub type EXTI12WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12WFL`"]
pub struct EXTI12WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12WFL_W<'a> {
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
#[doc = "Reader of field `EXTI13WFL`"]
pub type EXTI13WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13WFL`"]
pub struct EXTI13WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13WFL_W<'a> {
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
#[doc = "Reader of field `EXTI14WFL`"]
pub type EXTI14WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14WFL`"]
pub struct EXTI14WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14WFL_W<'a> {
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
#[doc = "Reader of field `EXTI15WFL`"]
pub type EXTI15WFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15WFL`"]
pub struct EXTI15WFL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15WFL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    pub fn exti0wfl(&self) -> EXTI0WFL_R {
        EXTI0WFL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    pub fn exti1wfl(&self) -> EXTI1WFL_R {
        EXTI1WFL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    pub fn exti2wfl(&self) -> EXTI2WFL_R {
        EXTI2WFL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    pub fn exti3wfl(&self) -> EXTI3WFL_R {
        EXTI3WFL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    pub fn exti4wfl(&self) -> EXTI4WFL_R {
        EXTI4WFL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    pub fn exti5wfl(&self) -> EXTI5WFL_R {
        EXTI5WFL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    pub fn exti6wfl(&self) -> EXTI6WFL_R {
        EXTI6WFL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    pub fn exti7wfl(&self) -> EXTI7WFL_R {
        EXTI7WFL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    pub fn exti8wfl(&self) -> EXTI8WFL_R {
        EXTI8WFL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    pub fn exti9wfl(&self) -> EXTI9WFL_R {
        EXTI9WFL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    pub fn exti10wfl(&self) -> EXTI10WFL_R {
        EXTI10WFL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    pub fn exti11wfl(&self) -> EXTI11WFL_R {
        EXTI11WFL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    pub fn exti12wfl(&self) -> EXTI12WFL_R {
        EXTI12WFL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    pub fn exti13wfl(&self) -> EXTI13WFL_R {
        EXTI13WFL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    pub fn exti14wfl(&self) -> EXTI14WFL_R {
        EXTI14WFL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    pub fn exti15wfl(&self) -> EXTI15WFL_R {
        EXTI15WFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WFL"]
    #[inline(always)]
    pub fn exti0wfl(&mut self) -> EXTI0WFL_W {
        EXTI0WFL_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1WFL"]
    #[inline(always)]
    pub fn exti1wfl(&mut self) -> EXTI1WFL_W {
        EXTI1WFL_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2WFL"]
    #[inline(always)]
    pub fn exti2wfl(&mut self) -> EXTI2WFL_W {
        EXTI2WFL_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3WFL"]
    #[inline(always)]
    pub fn exti3wfl(&mut self) -> EXTI3WFL_W {
        EXTI3WFL_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4WFL"]
    #[inline(always)]
    pub fn exti4wfl(&mut self) -> EXTI4WFL_W {
        EXTI4WFL_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5WFL"]
    #[inline(always)]
    pub fn exti5wfl(&mut self) -> EXTI5WFL_W {
        EXTI5WFL_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6WFL"]
    #[inline(always)]
    pub fn exti6wfl(&mut self) -> EXTI6WFL_W {
        EXTI6WFL_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7WFL"]
    #[inline(always)]
    pub fn exti7wfl(&mut self) -> EXTI7WFL_W {
        EXTI7WFL_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8WFL"]
    #[inline(always)]
    pub fn exti8wfl(&mut self) -> EXTI8WFL_W {
        EXTI8WFL_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9WFL"]
    #[inline(always)]
    pub fn exti9wfl(&mut self) -> EXTI9WFL_W {
        EXTI9WFL_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10WFL"]
    #[inline(always)]
    pub fn exti10wfl(&mut self) -> EXTI10WFL_W {
        EXTI10WFL_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11WFL"]
    #[inline(always)]
    pub fn exti11wfl(&mut self) -> EXTI11WFL_W {
        EXTI11WFL_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12WFL"]
    #[inline(always)]
    pub fn exti12wfl(&mut self) -> EXTI12WFL_W {
        EXTI12WFL_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13WFL"]
    #[inline(always)]
    pub fn exti13wfl(&mut self) -> EXTI13WFL_W {
        EXTI13WFL_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14WFL"]
    #[inline(always)]
    pub fn exti14wfl(&mut self) -> EXTI14WFL_W {
        EXTI14WFL_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15WFL"]
    #[inline(always)]
    pub fn exti15wfl(&mut self) -> EXTI15WFL_W {
        EXTI15WFL_W { w: self }
    }
}
