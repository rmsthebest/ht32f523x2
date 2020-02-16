#[doc = "Reader of register PWRCU_BAKSR"]
pub type R = crate::R<u32, super::PWRCU_BAKSR>;
#[doc = "Writer for register PWRCU_BAKSR"]
pub type W = crate::W<u32, super::PWRCU_BAKSR>;
#[doc = "Register PWRCU_BAKSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_BAKSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAKPORF`"]
pub type BAKPORF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAKPORF`"]
pub struct BAKPORF_W<'a> {
    w: &'a mut W,
}
impl<'a> BAKPORF_W<'a> {
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
#[doc = "Reader of field `PDF`"]
pub type PDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDF`"]
pub struct PDF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDF_W<'a> {
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
#[doc = "Reader of field `WUPF`"]
pub type WUPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPF`"]
pub struct WUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    pub fn bakporf(&self) -> BAKPORF_R {
        BAKPORF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    pub fn pdf(&self) -> PDF_R {
        PDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    pub fn wupf(&self) -> WUPF_R {
        WUPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    pub fn bakporf(&mut self) -> BAKPORF_W {
        BAKPORF_W { w: self }
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    pub fn pdf(&mut self) -> PDF_W {
        PDF_W { w: self }
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    pub fn wupf(&mut self) -> WUPF_W {
        WUPF_W { w: self }
    }
}
