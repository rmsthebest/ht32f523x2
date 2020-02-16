#[doc = "Reader of register EDGEFLGR"]
pub type R = crate::R<u32, super::EDGEFLGR>;
#[doc = "Writer for register EDGEFLGR"]
pub type W = crate::W<u32, super::EDGEFLGR>;
#[doc = "Register EDGEFLGR `reset()`'s with value 0"]
impl crate::ResetValue for super::EDGEFLGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0EDF`"]
pub type EXTI0EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0EDF`"]
pub struct EXTI0EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0EDF_W<'a> {
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
#[doc = "Reader of field `EXTI1EDF`"]
pub type EXTI1EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1EDF`"]
pub struct EXTI1EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1EDF_W<'a> {
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
#[doc = "Reader of field `EXTI2EDF`"]
pub type EXTI2EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2EDF`"]
pub struct EXTI2EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2EDF_W<'a> {
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
#[doc = "Reader of field `EXTI3EDF`"]
pub type EXTI3EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3EDF`"]
pub struct EXTI3EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3EDF_W<'a> {
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
#[doc = "Reader of field `EXTI4EDF`"]
pub type EXTI4EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4EDF`"]
pub struct EXTI4EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4EDF_W<'a> {
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
#[doc = "Reader of field `EXTI5EDF`"]
pub type EXTI5EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5EDF`"]
pub struct EXTI5EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5EDF_W<'a> {
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
#[doc = "Reader of field `EXTI6EDF`"]
pub type EXTI6EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6EDF`"]
pub struct EXTI6EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6EDF_W<'a> {
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
#[doc = "Reader of field `EXTI7EDF`"]
pub type EXTI7EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7EDF`"]
pub struct EXTI7EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7EDF_W<'a> {
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
#[doc = "Reader of field `EXTI8EDF`"]
pub type EXTI8EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8EDF`"]
pub struct EXTI8EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8EDF_W<'a> {
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
#[doc = "Reader of field `EXTI9EDF`"]
pub type EXTI9EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9EDF`"]
pub struct EXTI9EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9EDF_W<'a> {
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
#[doc = "Reader of field `EXTI10EDF`"]
pub type EXTI10EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10EDF`"]
pub struct EXTI10EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10EDF_W<'a> {
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
#[doc = "Reader of field `EXTI11EDF`"]
pub type EXTI11EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11EDF`"]
pub struct EXTI11EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11EDF_W<'a> {
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
#[doc = "Reader of field `EXTI12EDF`"]
pub type EXTI12EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12EDF`"]
pub struct EXTI12EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12EDF_W<'a> {
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
#[doc = "Reader of field `EXTI13EDF`"]
pub type EXTI13EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13EDF`"]
pub struct EXTI13EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13EDF_W<'a> {
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
#[doc = "Reader of field `EXTI14EDF`"]
pub type EXTI14EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14EDF`"]
pub struct EXTI14EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14EDF_W<'a> {
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
#[doc = "Reader of field `EXTI15EDF`"]
pub type EXTI15EDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15EDF`"]
pub struct EXTI15EDF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15EDF_W<'a> {
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
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    pub fn exti0edf(&self) -> EXTI0EDF_R {
        EXTI0EDF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    pub fn exti1edf(&self) -> EXTI1EDF_R {
        EXTI1EDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    pub fn exti2edf(&self) -> EXTI2EDF_R {
        EXTI2EDF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    pub fn exti3edf(&self) -> EXTI3EDF_R {
        EXTI3EDF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    pub fn exti4edf(&self) -> EXTI4EDF_R {
        EXTI4EDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    pub fn exti5edf(&self) -> EXTI5EDF_R {
        EXTI5EDF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    pub fn exti6edf(&self) -> EXTI6EDF_R {
        EXTI6EDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    pub fn exti7edf(&self) -> EXTI7EDF_R {
        EXTI7EDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    pub fn exti8edf(&self) -> EXTI8EDF_R {
        EXTI8EDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    pub fn exti9edf(&self) -> EXTI9EDF_R {
        EXTI9EDF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    pub fn exti10edf(&self) -> EXTI10EDF_R {
        EXTI10EDF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    pub fn exti11edf(&self) -> EXTI11EDF_R {
        EXTI11EDF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    pub fn exti12edf(&self) -> EXTI12EDF_R {
        EXTI12EDF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    pub fn exti13edf(&self) -> EXTI13EDF_R {
        EXTI13EDF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    pub fn exti14edf(&self) -> EXTI14EDF_R {
        EXTI14EDF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    pub fn exti15edf(&self) -> EXTI15EDF_R {
        EXTI15EDF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDF"]
    #[inline(always)]
    pub fn exti0edf(&mut self) -> EXTI0EDF_W {
        EXTI0EDF_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1EDF"]
    #[inline(always)]
    pub fn exti1edf(&mut self) -> EXTI1EDF_W {
        EXTI1EDF_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2EDF"]
    #[inline(always)]
    pub fn exti2edf(&mut self) -> EXTI2EDF_W {
        EXTI2EDF_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3EDF"]
    #[inline(always)]
    pub fn exti3edf(&mut self) -> EXTI3EDF_W {
        EXTI3EDF_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4EDF"]
    #[inline(always)]
    pub fn exti4edf(&mut self) -> EXTI4EDF_W {
        EXTI4EDF_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5EDF"]
    #[inline(always)]
    pub fn exti5edf(&mut self) -> EXTI5EDF_W {
        EXTI5EDF_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6EDF"]
    #[inline(always)]
    pub fn exti6edf(&mut self) -> EXTI6EDF_W {
        EXTI6EDF_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7EDF"]
    #[inline(always)]
    pub fn exti7edf(&mut self) -> EXTI7EDF_W {
        EXTI7EDF_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8EDF"]
    #[inline(always)]
    pub fn exti8edf(&mut self) -> EXTI8EDF_W {
        EXTI8EDF_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9EDF"]
    #[inline(always)]
    pub fn exti9edf(&mut self) -> EXTI9EDF_W {
        EXTI9EDF_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10EDF"]
    #[inline(always)]
    pub fn exti10edf(&mut self) -> EXTI10EDF_W {
        EXTI10EDF_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11EDF"]
    #[inline(always)]
    pub fn exti11edf(&mut self) -> EXTI11EDF_W {
        EXTI11EDF_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12EDF"]
    #[inline(always)]
    pub fn exti12edf(&mut self) -> EXTI12EDF_W {
        EXTI12EDF_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13EDF"]
    #[inline(always)]
    pub fn exti13edf(&mut self) -> EXTI13EDF_W {
        EXTI13EDF_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14EDF"]
    #[inline(always)]
    pub fn exti14edf(&mut self) -> EXTI14EDF_W {
        EXTI14EDF_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15EDF"]
    #[inline(always)]
    pub fn exti15edf(&mut self) -> EXTI15EDF_W {
        EXTI15EDF_W { w: self }
    }
}
