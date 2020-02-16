#[doc = "Reader of register WAKUPPOLR"]
pub type R = crate::R<u32, super::WAKUPPOLR>;
#[doc = "Writer for register WAKUPPOLR"]
pub type W = crate::W<u32, super::WAKUPPOLR>;
#[doc = "Register WAKUPPOLR `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKUPPOLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0WPOL`"]
pub type EXTI0WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0WPOL`"]
pub struct EXTI0WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI1WPOL`"]
pub type EXTI1WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1WPOL`"]
pub struct EXTI1WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI2WPOL`"]
pub type EXTI2WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2WPOL`"]
pub struct EXTI2WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI3WPOL`"]
pub type EXTI3WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3WPOL`"]
pub struct EXTI3WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI4WPOL`"]
pub type EXTI4WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4WPOL`"]
pub struct EXTI4WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI5WPOL`"]
pub type EXTI5WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5WPOL`"]
pub struct EXTI5WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI6WPOL`"]
pub type EXTI6WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6WPOL`"]
pub struct EXTI6WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI7WPOL`"]
pub type EXTI7WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7WPOL`"]
pub struct EXTI7WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI8WPOL`"]
pub type EXTI8WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8WPOL`"]
pub struct EXTI8WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI9WPOL`"]
pub type EXTI9WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9WPOL`"]
pub struct EXTI9WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI10WPOL`"]
pub type EXTI10WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10WPOL`"]
pub struct EXTI10WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI11WPOL`"]
pub type EXTI11WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11WPOL`"]
pub struct EXTI11WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI12WPOL`"]
pub type EXTI12WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12WPOL`"]
pub struct EXTI12WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI13WPOL`"]
pub type EXTI13WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13WPOL`"]
pub struct EXTI13WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI14WPOL`"]
pub type EXTI14WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14WPOL`"]
pub struct EXTI14WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14WPOL_W<'a> {
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
#[doc = "Reader of field `EXTI15WPOL`"]
pub type EXTI15WPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15WPOL`"]
pub struct EXTI15WPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15WPOL_W<'a> {
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
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    pub fn exti0wpol(&self) -> EXTI0WPOL_R {
        EXTI0WPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    pub fn exti1wpol(&self) -> EXTI1WPOL_R {
        EXTI1WPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    pub fn exti2wpol(&self) -> EXTI2WPOL_R {
        EXTI2WPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    pub fn exti3wpol(&self) -> EXTI3WPOL_R {
        EXTI3WPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    pub fn exti4wpol(&self) -> EXTI4WPOL_R {
        EXTI4WPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    pub fn exti5wpol(&self) -> EXTI5WPOL_R {
        EXTI5WPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    pub fn exti6wpol(&self) -> EXTI6WPOL_R {
        EXTI6WPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    pub fn exti7wpol(&self) -> EXTI7WPOL_R {
        EXTI7WPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    pub fn exti8wpol(&self) -> EXTI8WPOL_R {
        EXTI8WPOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    pub fn exti9wpol(&self) -> EXTI9WPOL_R {
        EXTI9WPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    pub fn exti10wpol(&self) -> EXTI10WPOL_R {
        EXTI10WPOL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    pub fn exti11wpol(&self) -> EXTI11WPOL_R {
        EXTI11WPOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    pub fn exti12wpol(&self) -> EXTI12WPOL_R {
        EXTI12WPOL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    pub fn exti13wpol(&self) -> EXTI13WPOL_R {
        EXTI13WPOL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    pub fn exti14wpol(&self) -> EXTI14WPOL_R {
        EXTI14WPOL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    pub fn exti15wpol(&self) -> EXTI15WPOL_R {
        EXTI15WPOL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0WPOL"]
    #[inline(always)]
    pub fn exti0wpol(&mut self) -> EXTI0WPOL_W {
        EXTI0WPOL_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1WPOL"]
    #[inline(always)]
    pub fn exti1wpol(&mut self) -> EXTI1WPOL_W {
        EXTI1WPOL_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2WPOL"]
    #[inline(always)]
    pub fn exti2wpol(&mut self) -> EXTI2WPOL_W {
        EXTI2WPOL_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3WPOL"]
    #[inline(always)]
    pub fn exti3wpol(&mut self) -> EXTI3WPOL_W {
        EXTI3WPOL_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4WPOL"]
    #[inline(always)]
    pub fn exti4wpol(&mut self) -> EXTI4WPOL_W {
        EXTI4WPOL_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5WPOL"]
    #[inline(always)]
    pub fn exti5wpol(&mut self) -> EXTI5WPOL_W {
        EXTI5WPOL_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6WPOL"]
    #[inline(always)]
    pub fn exti6wpol(&mut self) -> EXTI6WPOL_W {
        EXTI6WPOL_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7WPOL"]
    #[inline(always)]
    pub fn exti7wpol(&mut self) -> EXTI7WPOL_W {
        EXTI7WPOL_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8WPOL"]
    #[inline(always)]
    pub fn exti8wpol(&mut self) -> EXTI8WPOL_W {
        EXTI8WPOL_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9WPOL"]
    #[inline(always)]
    pub fn exti9wpol(&mut self) -> EXTI9WPOL_W {
        EXTI9WPOL_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10WPOL"]
    #[inline(always)]
    pub fn exti10wpol(&mut self) -> EXTI10WPOL_W {
        EXTI10WPOL_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11WPOL"]
    #[inline(always)]
    pub fn exti11wpol(&mut self) -> EXTI11WPOL_W {
        EXTI11WPOL_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12WPOL"]
    #[inline(always)]
    pub fn exti12wpol(&mut self) -> EXTI12WPOL_W {
        EXTI12WPOL_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13WPOL"]
    #[inline(always)]
    pub fn exti13wpol(&mut self) -> EXTI13WPOL_W {
        EXTI13WPOL_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14WPOL"]
    #[inline(always)]
    pub fn exti14wpol(&mut self) -> EXTI14WPOL_W {
        EXTI14WPOL_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15WPOL"]
    #[inline(always)]
    pub fn exti15wpol(&mut self) -> EXTI15WPOL_W {
        EXTI15WPOL_W { w: self }
    }
}
