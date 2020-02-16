#[doc = "Reader of register CNTCFR"]
pub type R = crate::R<u32, super::CNTCFR>;
#[doc = "Writer for register CNTCFR"]
pub type W = crate::W<u32, super::CNTCFR>;
#[doc = "Register CNTCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UEVDIS`"]
pub type UEVDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEVDIS`"]
pub struct UEVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UEVDIS_W<'a> {
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
#[doc = "Reader of field `UGDIS`"]
pub type UGDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UGDIS`"]
pub struct UGDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UGDIS_W<'a> {
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
#[doc = "Reader of field `CKDIV`"]
pub type CKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKDIV`"]
pub struct CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMSEL`"]
pub type CMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMSEL`"]
pub struct CMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&self) -> UEVDIS_R {
        UEVDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&self) -> UGDIS_R {
        UGDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    pub fn cmsel(&self) -> CMSEL_R {
        CMSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&mut self) -> UEVDIS_W {
        UEVDIS_W { w: self }
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&mut self) -> UGDIS_W {
        UGDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W { w: self }
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    pub fn cmsel(&mut self) -> CMSEL_W {
        CMSEL_W { w: self }
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
}
