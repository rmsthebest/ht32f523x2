#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLK`"]
pub type CCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK`"]
pub struct CCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_W<'a> {
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
#[doc = "Reader of field `CDIO`"]
pub type CDIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDIO`"]
pub struct CDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CDIO_W<'a> {
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
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
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
impl R {
    #[doc = "Bit 2 - CCLK"]
    #[inline(always)]
    pub fn cclk(&self) -> CCLK_R {
        CCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CDIO"]
    #[inline(always)]
    pub fn cdio(&self) -> CDIO_R {
        CDIO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CLKSEL"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CCLK"]
    #[inline(always)]
    pub fn cclk(&mut self) -> CCLK_W {
        CCLK_W { w: self }
    }
    #[doc = "Bit 3 - CDIO"]
    #[inline(always)]
    pub fn cdio(&mut self) -> CDIO_W {
        CDIO_W { w: self }
    }
    #[doc = "Bit 7 - CLKSEL"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
}
