#[doc = "Reader of register SSCR"]
pub type R = crate::R<u32, super::SSCR>;
#[doc = "Writer for register SSCR"]
pub type W = crate::W<u32, super::SSCR>;
#[doc = "Register SSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0SC`"]
pub type EXTI0SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0SC`"]
pub struct EXTI0SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0SC_W<'a> {
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
#[doc = "Reader of field `EXTI1SC`"]
pub type EXTI1SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1SC`"]
pub struct EXTI1SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1SC_W<'a> {
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
#[doc = "Reader of field `EXTI2SC`"]
pub type EXTI2SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2SC`"]
pub struct EXTI2SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2SC_W<'a> {
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
#[doc = "Reader of field `EXTI3SC`"]
pub type EXTI3SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3SC`"]
pub struct EXTI3SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3SC_W<'a> {
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
#[doc = "Reader of field `EXTI4SC`"]
pub type EXTI4SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4SC`"]
pub struct EXTI4SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4SC_W<'a> {
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
#[doc = "Reader of field `EXTI5SC`"]
pub type EXTI5SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5SC`"]
pub struct EXTI5SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5SC_W<'a> {
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
#[doc = "Reader of field `EXTI6SC`"]
pub type EXTI6SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6SC`"]
pub struct EXTI6SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6SC_W<'a> {
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
#[doc = "Reader of field `EXTI7SC`"]
pub type EXTI7SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7SC`"]
pub struct EXTI7SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7SC_W<'a> {
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
#[doc = "Reader of field `EXTI8SC`"]
pub type EXTI8SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8SC`"]
pub struct EXTI8SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8SC_W<'a> {
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
#[doc = "Reader of field `EXTI9SC`"]
pub type EXTI9SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9SC`"]
pub struct EXTI9SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9SC_W<'a> {
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
#[doc = "Reader of field `EXTI10SC`"]
pub type EXTI10SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10SC`"]
pub struct EXTI10SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10SC_W<'a> {
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
#[doc = "Reader of field `EXTI11SC`"]
pub type EXTI11SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11SC`"]
pub struct EXTI11SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11SC_W<'a> {
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
#[doc = "Reader of field `EXTI12SC`"]
pub type EXTI12SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12SC`"]
pub struct EXTI12SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12SC_W<'a> {
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
#[doc = "Reader of field `EXTI13SC`"]
pub type EXTI13SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13SC`"]
pub struct EXTI13SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13SC_W<'a> {
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
#[doc = "Reader of field `EXTI14SC`"]
pub type EXTI14SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14SC`"]
pub struct EXTI14SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14SC_W<'a> {
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
#[doc = "Reader of field `EXTI15SC`"]
pub type EXTI15SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15SC`"]
pub struct EXTI15SC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15SC_W<'a> {
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
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    pub fn exti0sc(&self) -> EXTI0SC_R {
        EXTI0SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    pub fn exti1sc(&self) -> EXTI1SC_R {
        EXTI1SC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    pub fn exti2sc(&self) -> EXTI2SC_R {
        EXTI2SC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    pub fn exti3sc(&self) -> EXTI3SC_R {
        EXTI3SC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    pub fn exti4sc(&self) -> EXTI4SC_R {
        EXTI4SC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    pub fn exti5sc(&self) -> EXTI5SC_R {
        EXTI5SC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    pub fn exti6sc(&self) -> EXTI6SC_R {
        EXTI6SC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    pub fn exti7sc(&self) -> EXTI7SC_R {
        EXTI7SC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    pub fn exti8sc(&self) -> EXTI8SC_R {
        EXTI8SC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    pub fn exti9sc(&self) -> EXTI9SC_R {
        EXTI9SC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    pub fn exti10sc(&self) -> EXTI10SC_R {
        EXTI10SC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    pub fn exti11sc(&self) -> EXTI11SC_R {
        EXTI11SC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    pub fn exti12sc(&self) -> EXTI12SC_R {
        EXTI12SC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    pub fn exti13sc(&self) -> EXTI13SC_R {
        EXTI13SC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    pub fn exti14sc(&self) -> EXTI14SC_R {
        EXTI14SC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    pub fn exti15sc(&self) -> EXTI15SC_R {
        EXTI15SC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0SC"]
    #[inline(always)]
    pub fn exti0sc(&mut self) -> EXTI0SC_W {
        EXTI0SC_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1SC"]
    #[inline(always)]
    pub fn exti1sc(&mut self) -> EXTI1SC_W {
        EXTI1SC_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2SC"]
    #[inline(always)]
    pub fn exti2sc(&mut self) -> EXTI2SC_W {
        EXTI2SC_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3SC"]
    #[inline(always)]
    pub fn exti3sc(&mut self) -> EXTI3SC_W {
        EXTI3SC_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4SC"]
    #[inline(always)]
    pub fn exti4sc(&mut self) -> EXTI4SC_W {
        EXTI4SC_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5SC"]
    #[inline(always)]
    pub fn exti5sc(&mut self) -> EXTI5SC_W {
        EXTI5SC_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6SC"]
    #[inline(always)]
    pub fn exti6sc(&mut self) -> EXTI6SC_W {
        EXTI6SC_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7SC"]
    #[inline(always)]
    pub fn exti7sc(&mut self) -> EXTI7SC_W {
        EXTI7SC_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8SC"]
    #[inline(always)]
    pub fn exti8sc(&mut self) -> EXTI8SC_W {
        EXTI8SC_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9SC"]
    #[inline(always)]
    pub fn exti9sc(&mut self) -> EXTI9SC_W {
        EXTI9SC_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10SC"]
    #[inline(always)]
    pub fn exti10sc(&mut self) -> EXTI10SC_W {
        EXTI10SC_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11SC"]
    #[inline(always)]
    pub fn exti11sc(&mut self) -> EXTI11SC_W {
        EXTI11SC_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12SC"]
    #[inline(always)]
    pub fn exti12sc(&mut self) -> EXTI12SC_W {
        EXTI12SC_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13SC"]
    #[inline(always)]
    pub fn exti13sc(&mut self) -> EXTI13SC_W {
        EXTI13SC_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14SC"]
    #[inline(always)]
    pub fn exti14sc(&mut self) -> EXTI14SC_W {
        EXTI14SC_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15SC"]
    #[inline(always)]
    pub fn exti15sc(&mut self) -> EXTI15SC_W {
        EXTI15SC_W { w: self }
    }
}
