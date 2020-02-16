#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0EN`"]
pub type EXTI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0EN`"]
pub struct EXTI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0EN_W<'a> {
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
#[doc = "Reader of field `EXTI1EN`"]
pub type EXTI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1EN`"]
pub struct EXTI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1EN_W<'a> {
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
#[doc = "Reader of field `EXTI2EN`"]
pub type EXTI2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2EN`"]
pub struct EXTI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2EN_W<'a> {
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
#[doc = "Reader of field `EXTI3EN`"]
pub type EXTI3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3EN`"]
pub struct EXTI3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3EN_W<'a> {
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
#[doc = "Reader of field `EXTI4EN`"]
pub type EXTI4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4EN`"]
pub struct EXTI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4EN_W<'a> {
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
#[doc = "Reader of field `EXTI5EN`"]
pub type EXTI5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5EN`"]
pub struct EXTI5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5EN_W<'a> {
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
#[doc = "Reader of field `EXTI6EN`"]
pub type EXTI6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6EN`"]
pub struct EXTI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6EN_W<'a> {
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
#[doc = "Reader of field `EXTI7EN`"]
pub type EXTI7EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7EN`"]
pub struct EXTI7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7EN_W<'a> {
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
#[doc = "Reader of field `EXTI8EN`"]
pub type EXTI8EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8EN`"]
pub struct EXTI8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8EN_W<'a> {
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
#[doc = "Reader of field `EXTI9EN`"]
pub type EXTI9EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9EN`"]
pub struct EXTI9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9EN_W<'a> {
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
#[doc = "Reader of field `EXTI10EN`"]
pub type EXTI10EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10EN`"]
pub struct EXTI10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10EN_W<'a> {
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
#[doc = "Reader of field `EXTI11EN`"]
pub type EXTI11EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11EN`"]
pub struct EXTI11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11EN_W<'a> {
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
#[doc = "Reader of field `EXTI12EN`"]
pub type EXTI12EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12EN`"]
pub struct EXTI12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12EN_W<'a> {
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
#[doc = "Reader of field `EXTI13EN`"]
pub type EXTI13EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13EN`"]
pub struct EXTI13EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13EN_W<'a> {
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
#[doc = "Reader of field `EXTI14EN`"]
pub type EXTI14EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14EN`"]
pub struct EXTI14EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14EN_W<'a> {
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
#[doc = "Reader of field `EXTI15EN`"]
pub type EXTI15EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15EN`"]
pub struct EXTI15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15EN_W<'a> {
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
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    pub fn exti0en(&self) -> EXTI0EN_R {
        EXTI0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    pub fn exti1en(&self) -> EXTI1EN_R {
        EXTI1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    pub fn exti2en(&self) -> EXTI2EN_R {
        EXTI2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    pub fn exti3en(&self) -> EXTI3EN_R {
        EXTI3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    pub fn exti4en(&self) -> EXTI4EN_R {
        EXTI4EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    pub fn exti5en(&self) -> EXTI5EN_R {
        EXTI5EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    pub fn exti6en(&self) -> EXTI6EN_R {
        EXTI6EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    pub fn exti7en(&self) -> EXTI7EN_R {
        EXTI7EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    pub fn exti8en(&self) -> EXTI8EN_R {
        EXTI8EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    pub fn exti9en(&self) -> EXTI9EN_R {
        EXTI9EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    pub fn exti10en(&self) -> EXTI10EN_R {
        EXTI10EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    pub fn exti11en(&self) -> EXTI11EN_R {
        EXTI11EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    pub fn exti12en(&self) -> EXTI12EN_R {
        EXTI12EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    pub fn exti13en(&self) -> EXTI13EN_R {
        EXTI13EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    pub fn exti14en(&self) -> EXTI14EN_R {
        EXTI14EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    pub fn exti15en(&self) -> EXTI15EN_R {
        EXTI15EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0EN"]
    #[inline(always)]
    pub fn exti0en(&mut self) -> EXTI0EN_W {
        EXTI0EN_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1EN"]
    #[inline(always)]
    pub fn exti1en(&mut self) -> EXTI1EN_W {
        EXTI1EN_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2EN"]
    #[inline(always)]
    pub fn exti2en(&mut self) -> EXTI2EN_W {
        EXTI2EN_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3EN"]
    #[inline(always)]
    pub fn exti3en(&mut self) -> EXTI3EN_W {
        EXTI3EN_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4EN"]
    #[inline(always)]
    pub fn exti4en(&mut self) -> EXTI4EN_W {
        EXTI4EN_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5EN"]
    #[inline(always)]
    pub fn exti5en(&mut self) -> EXTI5EN_W {
        EXTI5EN_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6EN"]
    #[inline(always)]
    pub fn exti6en(&mut self) -> EXTI6EN_W {
        EXTI6EN_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7EN"]
    #[inline(always)]
    pub fn exti7en(&mut self) -> EXTI7EN_W {
        EXTI7EN_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8EN"]
    #[inline(always)]
    pub fn exti8en(&mut self) -> EXTI8EN_W {
        EXTI8EN_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9EN"]
    #[inline(always)]
    pub fn exti9en(&mut self) -> EXTI9EN_W {
        EXTI9EN_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10EN"]
    #[inline(always)]
    pub fn exti10en(&mut self) -> EXTI10EN_W {
        EXTI10EN_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11EN"]
    #[inline(always)]
    pub fn exti11en(&mut self) -> EXTI11EN_W {
        EXTI11EN_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12EN"]
    #[inline(always)]
    pub fn exti12en(&mut self) -> EXTI12EN_W {
        EXTI12EN_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13EN"]
    #[inline(always)]
    pub fn exti13en(&mut self) -> EXTI13EN_W {
        EXTI13EN_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14EN"]
    #[inline(always)]
    pub fn exti14en(&mut self) -> EXTI14EN_W {
        EXTI14EN_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15EN"]
    #[inline(always)]
    pub fn exti15en(&mut self) -> EXTI15EN_W {
        EXTI15EN_W { w: self }
    }
}
