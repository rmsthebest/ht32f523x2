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
#[doc = "Reader of field `MIEN`"]
pub type MIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIEN`"]
pub struct MIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIEN_W<'a> {
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
#[doc = "Reader of field `OSM`"]
pub type OSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSM`"]
pub struct OSM_W<'a> {
    w: &'a mut W,
}
impl<'a> OSM_W<'a> {
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
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MIEN"]
    #[inline(always)]
    pub fn mien(&self) -> MIEN_R {
        MIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OSM"]
    #[inline(always)]
    pub fn osm(&self) -> OSM_R {
        OSM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MIEN"]
    #[inline(always)]
    pub fn mien(&mut self) -> MIEN_W {
        MIEN_W { w: self }
    }
    #[doc = "Bit 1 - OSM"]
    #[inline(always)]
    pub fn osm(&mut self) -> OSM_W {
        OSM_W { w: self }
    }
    #[doc = "Bit 2 - CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
}
