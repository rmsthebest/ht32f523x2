#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFIF`"]
pub type SOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIF`"]
pub struct SOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIF_W<'a> {
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
#[doc = "Reader of field `URSTIF`"]
pub type URSTIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URSTIF`"]
pub struct URSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIF_W<'a> {
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
#[doc = "Reader of field `RSMIF`"]
pub type RSMIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSMIF`"]
pub struct RSMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMIF_W<'a> {
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
#[doc = "Reader of field `SUSPIF`"]
pub type SUSPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPIF`"]
pub struct SUSPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPIF_W<'a> {
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
#[doc = "Reader of field `ESOFIF`"]
pub type ESOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESOFIF`"]
pub struct ESOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFIF_W<'a> {
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
#[doc = "Reader of field `EP0IF`"]
pub type EP0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0IF`"]
pub struct EP0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IF_W<'a> {
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
#[doc = "Reader of field `EP1IF`"]
pub type EP1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1IF`"]
pub struct EP1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1IF_W<'a> {
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
#[doc = "Reader of field `EP2IF`"]
pub type EP2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2IF`"]
pub struct EP2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2IF_W<'a> {
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
#[doc = "Reader of field `EP3IF`"]
pub type EP3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3IF`"]
pub struct EP3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3IF_W<'a> {
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
#[doc = "Reader of field `EP4IF`"]
pub type EP4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4IF`"]
pub struct EP4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4IF_W<'a> {
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
#[doc = "Reader of field `EP5IF`"]
pub type EP5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5IF`"]
pub struct EP5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5IF_W<'a> {
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
#[doc = "Reader of field `EP6IF`"]
pub type EP6IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6IF`"]
pub struct EP6IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6IF_W<'a> {
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
#[doc = "Reader of field `EP7IF`"]
pub type EP7IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7IF`"]
pub struct EP7IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7IF_W<'a> {
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
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    pub fn urstif(&self) -> URSTIF_R {
        URSTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    pub fn rsmif(&self) -> RSMIF_R {
        RSMIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    pub fn suspif(&self) -> SUSPIF_R {
        SUSPIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    pub fn ep0if(&self) -> EP0IF_R {
        EP0IF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    pub fn ep1if(&self) -> EP1IF_R {
        EP1IF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    pub fn ep2if(&self) -> EP2IF_R {
        EP2IF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    pub fn ep3if(&self) -> EP3IF_R {
        EP3IF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    pub fn ep4if(&self) -> EP4IF_R {
        EP4IF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    pub fn ep5if(&self) -> EP5IF_R {
        EP5IF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    pub fn ep6if(&self) -> EP6IF_R {
        EP6IF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    pub fn ep7if(&self) -> EP7IF_R {
        EP7IF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W { w: self }
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    pub fn urstif(&mut self) -> URSTIF_W {
        URSTIF_W { w: self }
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    pub fn rsmif(&mut self) -> RSMIF_W {
        RSMIF_W { w: self }
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    pub fn suspif(&mut self) -> SUSPIF_W {
        SUSPIF_W { w: self }
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    pub fn esofif(&mut self) -> ESOFIF_W {
        ESOFIF_W { w: self }
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    pub fn ep0if(&mut self) -> EP0IF_W {
        EP0IF_W { w: self }
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    pub fn ep1if(&mut self) -> EP1IF_W {
        EP1IF_W { w: self }
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    pub fn ep2if(&mut self) -> EP2IF_W {
        EP2IF_W { w: self }
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    pub fn ep3if(&mut self) -> EP3IF_W {
        EP3IF_W { w: self }
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    pub fn ep4if(&mut self) -> EP4IF_W {
        EP4IF_W { w: self }
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    pub fn ep5if(&mut self) -> EP5IF_W {
        EP5IF_W { w: self }
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    pub fn ep6if(&mut self) -> EP6IF_W {
        EP6IF_W { w: self }
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    pub fn ep7if(&mut self) -> EP7IF_W {
        EP7IF_W { w: self }
    }
}
