#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EBIBUSY`"]
pub type EBIBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIBUSY`"]
pub struct EBIBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIBUSY_W<'a> {
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
#[doc = "Reader of field `EBISMRST`"]
pub type EBISMRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBISMRST`"]
pub struct EBISMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EBISMRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&self) -> EBIBUSY_R {
        EBIBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&self) -> EBISMRST_R {
        EBISMRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&mut self) -> EBIBUSY_W {
        EBIBUSY_W { w: self }
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&mut self) -> EBISMRST_W {
        EBISMRST_W { w: self }
    }
}
