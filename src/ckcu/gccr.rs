#[doc = "Reader of register GCCR"]
pub type R = crate::R<u32, super::GCCR>;
#[doc = "Writer for register GCCR"]
pub type W = crate::W<u32, super::GCCR>;
#[doc = "Register GCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HSEGAIN`"]
pub type HSEGAIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEGAIN`"]
pub struct HSEGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEGAIN_W<'a> {
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
#[doc = "Reader of field `PLLEN`"]
pub type PLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLEN`"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Reader of field `HSEEN`"]
pub type HSEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEEN`"]
pub struct HSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEEN_W<'a> {
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
#[doc = "Reader of field `HSIEN`"]
pub type HSIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIEN`"]
pub struct HSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIEN_W<'a> {
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
#[doc = "Reader of field `CKMEN`"]
pub type CKMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKMEN`"]
pub struct CKMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PSRCEN`"]
pub type PSRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSRCEN`"]
pub struct PSRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    pub fn hsegain(&self) -> HSEGAIN_R {
        HSEGAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    pub fn psrcen(&self) -> PSRCEN_R {
        PSRCEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    pub fn hsegain(&mut self) -> HSEGAIN_W {
        HSEGAIN_W { w: self }
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&mut self) -> HSEEN_W {
        HSEEN_W { w: self }
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&mut self) -> HSIEN_W {
        HSIEN_W { w: self }
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CKMEN_W {
        CKMEN_W { w: self }
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    pub fn psrcen(&mut self) -> PSRCEN_W {
        PSRCEN_W { w: self }
    }
}
