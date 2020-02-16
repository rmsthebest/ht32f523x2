#[doc = "Reader of register IRAW"]
pub type R = crate::R<u32, super::IRAW>;
#[doc = "Writer for register IRAW"]
pub type W = crate::W<u32, super::IRAW>;
#[doc = "Register IRAW `reset()`'s with value 0"]
impl crate::ResetValue for super::IRAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADIRAWS`"]
pub type ADIRAWS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWS`"]
pub struct ADIRAWS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWS_W<'a> {
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
#[doc = "Reader of field `ADIRAWG`"]
pub type ADIRAWG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWG`"]
pub struct ADIRAWG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWG_W<'a> {
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
#[doc = "Reader of field `ADIRAWC`"]
pub type ADIRAWC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWC`"]
pub struct ADIRAWC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWC_W<'a> {
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
#[doc = "Reader of field `ADIRAWL`"]
pub type ADIRAWL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWL`"]
pub struct ADIRAWL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWL_W<'a> {
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
#[doc = "Reader of field `ADIRAWU`"]
pub type ADIRAWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWU`"]
pub struct ADIRAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWU_W<'a> {
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
#[doc = "Reader of field `ADIRAWO`"]
pub type ADIRAWO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIRAWO`"]
pub struct ADIRAWO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIRAWO_W<'a> {
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
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    pub fn adiraws(&self) -> ADIRAWS_R {
        ADIRAWS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    pub fn adirawg(&self) -> ADIRAWG_R {
        ADIRAWG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    pub fn adirawc(&self) -> ADIRAWC_R {
        ADIRAWC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    pub fn adirawl(&self) -> ADIRAWL_R {
        ADIRAWL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    pub fn adirawu(&self) -> ADIRAWU_R {
        ADIRAWU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    pub fn adirawo(&self) -> ADIRAWO_R {
        ADIRAWO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    pub fn adiraws(&mut self) -> ADIRAWS_W {
        ADIRAWS_W { w: self }
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    pub fn adirawg(&mut self) -> ADIRAWG_W {
        ADIRAWG_W { w: self }
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    pub fn adirawc(&mut self) -> ADIRAWC_W {
        ADIRAWC_W { w: self }
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    pub fn adirawl(&mut self) -> ADIRAWL_W {
        ADIRAWL_W { w: self }
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    pub fn adirawu(&mut self) -> ADIRAWU_W {
        ADIRAWU_W { w: self }
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    pub fn adirawo(&mut self) -> ADIRAWO_W {
        ADIRAWO_W { w: self }
    }
}
