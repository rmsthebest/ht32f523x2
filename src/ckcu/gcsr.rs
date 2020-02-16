#[doc = "Reader of register GCSR"]
pub type R = crate::R<u32, super::GCSR>;
#[doc = "Writer for register GCSR"]
pub type W = crate::W<u32, super::GCSR>;
#[doc = "Register GCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLRDY`"]
pub type PLLRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLRDY`"]
pub struct PLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDY_W<'a> {
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
#[doc = "Reader of field `HSERDY`"]
pub type HSERDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSERDY`"]
pub struct HSERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDY_W<'a> {
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
#[doc = "Reader of field `HSIRDY`"]
pub type HSIRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIRDY`"]
pub struct HSIRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDY_W<'a> {
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
#[doc = "Reader of field `LSERDY`"]
pub type LSERDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSERDY`"]
pub struct LSERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDY_W<'a> {
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
#[doc = "Reader of field `LSIRDY`"]
pub type LSIRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSIRDY`"]
pub struct LSIRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDY_W<'a> {
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
impl R {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PLLRDY_W {
        PLLRDY_W { w: self }
    }
    #[doc = "Bit 2 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HSERDY_W {
        HSERDY_W { w: self }
    }
    #[doc = "Bit 3 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HSIRDY_W {
        HSIRDY_W { w: self }
    }
    #[doc = "Bit 4 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&mut self) -> LSERDY_W {
        LSERDY_W { w: self }
    }
    #[doc = "Bit 5 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LSIRDY_W {
        LSIRDY_W { w: self }
    }
}
