#[doc = "Reader of register EDGESR"]
pub type R = crate::R<u32, super::EDGESR>;
#[doc = "Writer for register EDGESR"]
pub type W = crate::W<u32, super::EDGESR>;
#[doc = "Register EDGESR `reset()`'s with value 0"]
impl crate::ResetValue for super::EDGESR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0EDS`"]
pub type EXTI0EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0EDS`"]
pub struct EXTI0EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0EDS_W<'a> {
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
#[doc = "Reader of field `EXTI1EDS`"]
pub type EXTI1EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1EDS`"]
pub struct EXTI1EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1EDS_W<'a> {
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
#[doc = "Reader of field `EXTI2EDS`"]
pub type EXTI2EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2EDS`"]
pub struct EXTI2EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2EDS_W<'a> {
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
#[doc = "Reader of field `EXTI3EDS`"]
pub type EXTI3EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3EDS`"]
pub struct EXTI3EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3EDS_W<'a> {
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
#[doc = "Reader of field `EXTI4EDS`"]
pub type EXTI4EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4EDS`"]
pub struct EXTI4EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4EDS_W<'a> {
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
#[doc = "Reader of field `EXTI5EDS`"]
pub type EXTI5EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5EDS`"]
pub struct EXTI5EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5EDS_W<'a> {
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
#[doc = "Reader of field `EXTI6EDS`"]
pub type EXTI6EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6EDS`"]
pub struct EXTI6EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6EDS_W<'a> {
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
#[doc = "Reader of field `EXTI7EDS`"]
pub type EXTI7EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7EDS`"]
pub struct EXTI7EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7EDS_W<'a> {
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
#[doc = "Reader of field `EXTI8EDS`"]
pub type EXTI8EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8EDS`"]
pub struct EXTI8EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8EDS_W<'a> {
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
#[doc = "Reader of field `EXTI9EDS`"]
pub type EXTI9EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9EDS`"]
pub struct EXTI9EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9EDS_W<'a> {
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
#[doc = "Reader of field `EXTI10EDS`"]
pub type EXTI10EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10EDS`"]
pub struct EXTI10EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10EDS_W<'a> {
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
#[doc = "Reader of field `EXTI11EDS`"]
pub type EXTI11EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11EDS`"]
pub struct EXTI11EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11EDS_W<'a> {
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
#[doc = "Reader of field `EXTI12EDS`"]
pub type EXTI12EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12EDS`"]
pub struct EXTI12EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12EDS_W<'a> {
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
#[doc = "Reader of field `EXTI13EDS`"]
pub type EXTI13EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13EDS`"]
pub struct EXTI13EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13EDS_W<'a> {
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
#[doc = "Reader of field `EXTI14EDS`"]
pub type EXTI14EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14EDS`"]
pub struct EXTI14EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14EDS_W<'a> {
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
#[doc = "Reader of field `EXTI15EDS`"]
pub type EXTI15EDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15EDS`"]
pub struct EXTI15EDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15EDS_W<'a> {
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
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    pub fn exti0eds(&self) -> EXTI0EDS_R {
        EXTI0EDS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    pub fn exti1eds(&self) -> EXTI1EDS_R {
        EXTI1EDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    pub fn exti2eds(&self) -> EXTI2EDS_R {
        EXTI2EDS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    pub fn exti3eds(&self) -> EXTI3EDS_R {
        EXTI3EDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    pub fn exti4eds(&self) -> EXTI4EDS_R {
        EXTI4EDS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    pub fn exti5eds(&self) -> EXTI5EDS_R {
        EXTI5EDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    pub fn exti6eds(&self) -> EXTI6EDS_R {
        EXTI6EDS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    pub fn exti7eds(&self) -> EXTI7EDS_R {
        EXTI7EDS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    pub fn exti8eds(&self) -> EXTI8EDS_R {
        EXTI8EDS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    pub fn exti9eds(&self) -> EXTI9EDS_R {
        EXTI9EDS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    pub fn exti10eds(&self) -> EXTI10EDS_R {
        EXTI10EDS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    pub fn exti11eds(&self) -> EXTI11EDS_R {
        EXTI11EDS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    pub fn exti12eds(&self) -> EXTI12EDS_R {
        EXTI12EDS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    pub fn exti13eds(&self) -> EXTI13EDS_R {
        EXTI13EDS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    pub fn exti14eds(&self) -> EXTI14EDS_R {
        EXTI14EDS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    pub fn exti15eds(&self) -> EXTI15EDS_R {
        EXTI15EDS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EDS"]
    #[inline(always)]
    pub fn exti0eds(&mut self) -> EXTI0EDS_W {
        EXTI0EDS_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1EDS"]
    #[inline(always)]
    pub fn exti1eds(&mut self) -> EXTI1EDS_W {
        EXTI1EDS_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2EDS"]
    #[inline(always)]
    pub fn exti2eds(&mut self) -> EXTI2EDS_W {
        EXTI2EDS_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3EDS"]
    #[inline(always)]
    pub fn exti3eds(&mut self) -> EXTI3EDS_W {
        EXTI3EDS_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4EDS"]
    #[inline(always)]
    pub fn exti4eds(&mut self) -> EXTI4EDS_W {
        EXTI4EDS_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5EDS"]
    #[inline(always)]
    pub fn exti5eds(&mut self) -> EXTI5EDS_W {
        EXTI5EDS_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6EDS"]
    #[inline(always)]
    pub fn exti6eds(&mut self) -> EXTI6EDS_W {
        EXTI6EDS_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7EDS"]
    #[inline(always)]
    pub fn exti7eds(&mut self) -> EXTI7EDS_W {
        EXTI7EDS_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8EDS"]
    #[inline(always)]
    pub fn exti8eds(&mut self) -> EXTI8EDS_W {
        EXTI8EDS_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9EDS"]
    #[inline(always)]
    pub fn exti9eds(&mut self) -> EXTI9EDS_W {
        EXTI9EDS_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10EDS"]
    #[inline(always)]
    pub fn exti10eds(&mut self) -> EXTI10EDS_W {
        EXTI10EDS_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11EDS"]
    #[inline(always)]
    pub fn exti11eds(&mut self) -> EXTI11EDS_W {
        EXTI11EDS_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12EDS"]
    #[inline(always)]
    pub fn exti12eds(&mut self) -> EXTI12EDS_W {
        EXTI12EDS_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13EDS"]
    #[inline(always)]
    pub fn exti13eds(&mut self) -> EXTI13EDS_W {
        EXTI13EDS_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14EDS"]
    #[inline(always)]
    pub fn exti14eds(&mut self) -> EXTI14EDS_W {
        EXTI14EDS_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15EDS"]
    #[inline(always)]
    pub fn exti15eds(&mut self) -> EXTI15EDS_W {
        EXTI15EDS_W { w: self }
    }
}
