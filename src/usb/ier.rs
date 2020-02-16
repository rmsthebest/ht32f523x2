#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UGIE`"]
pub type UGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UGIE`"]
pub struct UGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UGIE_W<'a> {
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
#[doc = "Reader of field `SOFIE`"]
pub type SOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIE`"]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
#[doc = "Reader of field `URSTIE`"]
pub type URSTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URSTIE`"]
pub struct URSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIE_W<'a> {
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
#[doc = "Reader of field `RSMIE`"]
pub type RSMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSMIE`"]
pub struct RSMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMIE_W<'a> {
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
#[doc = "Reader of field `SUSPIE`"]
pub type SUSPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPIE`"]
pub struct SUSPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPIE_W<'a> {
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
#[doc = "Reader of field `ESOFIE`"]
pub type ESOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESOFIE`"]
pub struct ESOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFIE_W<'a> {
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
#[doc = "Reader of field `EP0IE`"]
pub type EP0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0IE`"]
pub struct EP0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IE_W<'a> {
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
#[doc = "Reader of field `EP1IE`"]
pub type EP1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1IE`"]
pub struct EP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1IE_W<'a> {
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
#[doc = "Reader of field `EP2IE`"]
pub type EP2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2IE`"]
pub struct EP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2IE_W<'a> {
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
#[doc = "Reader of field `EP3IE`"]
pub type EP3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3IE`"]
pub struct EP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3IE_W<'a> {
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
#[doc = "Reader of field `EP4IE`"]
pub type EP4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4IE`"]
pub struct EP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4IE_W<'a> {
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
#[doc = "Reader of field `EP5IE`"]
pub type EP5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5IE`"]
pub struct EP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5IE_W<'a> {
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
#[doc = "Reader of field `EP6IE`"]
pub type EP6IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6IE`"]
pub struct EP6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6IE_W<'a> {
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
#[doc = "Reader of field `EP7IE`"]
pub type EP7IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7IE`"]
pub struct EP7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7IE_W<'a> {
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
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    pub fn ugie(&self) -> UGIE_R {
        UGIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    pub fn urstie(&self) -> URSTIE_R {
        URSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    pub fn rsmie(&self) -> RSMIE_R {
        RSMIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    pub fn ep0ie(&self) -> EP0IE_R {
        EP0IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    pub fn ep1ie(&self) -> EP1IE_R {
        EP1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    pub fn ep2ie(&self) -> EP2IE_R {
        EP2IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    pub fn ep3ie(&self) -> EP3IE_R {
        EP3IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    pub fn ep4ie(&self) -> EP4IE_R {
        EP4IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    pub fn ep5ie(&self) -> EP5IE_R {
        EP5IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    pub fn ep6ie(&self) -> EP6IE_R {
        EP6IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    pub fn ep7ie(&self) -> EP7IE_R {
        EP7IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    pub fn ugie(&mut self) -> UGIE_W {
        UGIE_W { w: self }
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    pub fn urstie(&mut self) -> URSTIE_W {
        URSTIE_W { w: self }
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    pub fn rsmie(&mut self) -> RSMIE_W {
        RSMIE_W { w: self }
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    pub fn suspie(&mut self) -> SUSPIE_W {
        SUSPIE_W { w: self }
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    pub fn esofie(&mut self) -> ESOFIE_W {
        ESOFIE_W { w: self }
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    pub fn ep0ie(&mut self) -> EP0IE_W {
        EP0IE_W { w: self }
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    pub fn ep1ie(&mut self) -> EP1IE_W {
        EP1IE_W { w: self }
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    pub fn ep2ie(&mut self) -> EP2IE_W {
        EP2IE_W { w: self }
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    pub fn ep3ie(&mut self) -> EP3IE_W {
        EP3IE_W { w: self }
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    pub fn ep4ie(&mut self) -> EP4IE_W {
        EP4IE_W { w: self }
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    pub fn ep5ie(&mut self) -> EP5IE_W {
        EP5IE_W { w: self }
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    pub fn ep6ie(&mut self) -> EP6IE_W {
        EP6IE_W { w: self }
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    pub fn ep7ie(&mut self) -> EP7IE_W {
        EP7IE_W { w: self }
    }
}
