#[doc = "Reader of register GRSR"]
pub type R = crate::R<u32, super::GRSR>;
#[doc = "Writer for register GRSR"]
pub type W = crate::W<u32, super::GRSR>;
#[doc = "Register GRSR `reset()`'s with value 0"]
impl crate::ResetValue for super::GRSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVICRSTF`"]
pub type NVICRSTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVICRSTF`"]
pub struct NVICRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> NVICRSTF_W<'a> {
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
#[doc = "Reader of field `EXTRSTF`"]
pub type EXTRSTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTRSTF`"]
pub struct EXTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRSTF_W<'a> {
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
#[doc = "Reader of field `WDTRSTF`"]
pub type WDTRSTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRSTF`"]
pub struct WDTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRSTF_W<'a> {
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
#[doc = "Reader of field `PORSTF`"]
pub type PORSTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORSTF`"]
pub struct PORSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORSTF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    pub fn nvicrstf(&self) -> NVICRSTF_R {
        NVICRSTF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    pub fn extrstf(&self) -> EXTRSTF_R {
        EXTRSTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WDTRSTF_R {
        WDTRSTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    pub fn porstf(&self) -> PORSTF_R {
        PORSTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    pub fn nvicrstf(&mut self) -> NVICRSTF_W {
        NVICRSTF_W { w: self }
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    pub fn extrstf(&mut self) -> EXTRSTF_W {
        EXTRSTF_W { w: self }
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    pub fn wdtrstf(&mut self) -> WDTRSTF_W {
        WDTRSTF_W { w: self }
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    pub fn porstf(&mut self) -> PORSTF_W {
        PORSTF_W { w: self }
    }
}
