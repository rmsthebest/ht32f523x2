#[doc = "Reader of register GPTM_MDCFR"]
pub type R = crate::R<u32, super::GPTM_MDCFR>;
#[doc = "Writer for register GPTM_MDCFR"]
pub type W = crate::W<u32, super::GPTM_MDCFR>;
#[doc = "Register GPTM_MDCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_MDCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSE`"]
pub type TSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE`"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
#[doc = "Reader of field `SMSEL`"]
pub type SMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMSEL`"]
pub struct SMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `MMSEL`"]
pub type MMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MMSEL`"]
pub struct MMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPMSET`"]
pub type SPMSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPMSET`"]
pub struct SPMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPMSET_W<'a> {
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
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&self) -> MMSEL_R {
        MMSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&self) -> SPMSET_R {
        SPMSET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&mut self) -> SMSEL_W {
        SMSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&mut self) -> MMSEL_W {
        MMSEL_W { w: self }
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&mut self) -> SPMSET_W {
        SPMSET_W { w: self }
    }
}
