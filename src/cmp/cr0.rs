#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
#[doc = "Reader of field `CMPSM`"]
pub type CMPSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPSM`"]
pub struct CMPSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSM_W<'a> {
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
#[doc = "Reader of field `CMPHM`"]
pub type CMPHM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPHM`"]
pub struct CMPHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPHM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CMPINSEL`"]
pub type CMPINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPINSEL`"]
pub struct CMPINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMPPOL`"]
pub type CMPPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPPOL`"]
pub struct CMPPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPOL_W<'a> {
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
#[doc = "Reader of field `SYNCSEL`"]
pub type SYNCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCSEL`"]
pub struct SYNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSEL_W<'a> {
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
#[doc = "Reader of field `CVREN`"]
pub type CVREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVREN`"]
pub struct CVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CVREN_W<'a> {
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
#[doc = "Reader of field `CVROE`"]
pub type CVROE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVROE`"]
pub struct CVROE_W<'a> {
    w: &'a mut W,
}
impl<'a> CVROE_W<'a> {
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
#[doc = "Reader of field `CVRSS`"]
pub type CVRSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVRSS`"]
pub struct CVRSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRSS_W<'a> {
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
#[doc = "Reader of field `CMPOSEL`"]
pub type CMPOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPOSEL`"]
pub struct CMPOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `CMPWPEN`"]
pub type CMPWPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPWPEN`"]
pub struct CMPWPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPWPEN_W<'a> {
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
#[doc = "Reader of field `CMPSTS`"]
pub type CMPSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPSTS`"]
pub struct CMPSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSTS_W<'a> {
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
#[doc = "Reader of field `PROTECT`"]
pub type PROTECT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROTECT`"]
pub struct PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    pub fn cmpsm(&self) -> CMPSM_R {
        CMPSM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    pub fn cmphm(&self) -> CMPHM_R {
        CMPHM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    pub fn cmpinsel(&self) -> CMPINSEL_R {
        CMPINSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    pub fn cmppol(&self) -> CMPPOL_R {
        CMPPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    pub fn cvren(&self) -> CVREN_R {
        CVREN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    pub fn cvroe(&self) -> CVROE_R {
        CVROE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CVRSS"]
    #[inline(always)]
    pub fn cvrss(&self) -> CVRSS_R {
        CVRSS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    pub fn cmposel(&self) -> CMPOSEL_R {
        CMPOSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    pub fn cmpwpen(&self) -> CMPWPEN_R {
        CMPWPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    pub fn cmpsts(&self) -> CMPSTS_R {
        CMPSTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    pub fn cmpsm(&mut self) -> CMPSM_W {
        CMPSM_W { w: self }
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    pub fn cmphm(&mut self) -> CMPHM_W {
        CMPHM_W { w: self }
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    pub fn cmpinsel(&mut self) -> CMPINSEL_W {
        CMPINSEL_W { w: self }
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    pub fn cmppol(&mut self) -> CMPPOL_W {
        CMPPOL_W { w: self }
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W {
        SYNCSEL_W { w: self }
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    pub fn cvren(&mut self) -> CVREN_W {
        CVREN_W { w: self }
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    pub fn cvroe(&mut self) -> CVROE_W {
        CVROE_W { w: self }
    }
    #[doc = "Bit 10 - CVRSS"]
    #[inline(always)]
    pub fn cvrss(&mut self) -> CVRSS_W {
        CVRSS_W { w: self }
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    pub fn cmposel(&mut self) -> CMPOSEL_W {
        CMPOSEL_W { w: self }
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    pub fn cmpwpen(&mut self) -> CMPWPEN_W {
        CMPWPEN_W { w: self }
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    pub fn cmpsts(&mut self) -> CMPSTS_W {
        CMPSTS_W { w: self }
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    pub fn protect(&mut self) -> PROTECT_W {
        PROTECT_W { w: self }
    }
}
