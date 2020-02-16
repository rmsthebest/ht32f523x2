#[doc = "Reader of register GPTM_CTR"]
pub type R = crate::R<u32, super::GPTM_CTR>;
#[doc = "Writer for register GPTM_CTR"]
pub type W = crate::W<u32, super::GPTM_CTR>;
#[doc = "Register GPTM_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TME`"]
pub type TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TME`"]
pub struct TME_W<'a> {
    w: &'a mut W,
}
impl<'a> TME_W<'a> {
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
#[doc = "Reader of field `CRBE`"]
pub type CRBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRBE`"]
pub struct CRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRBE_W<'a> {
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
#[doc = "Reader of field `CHCCDS`"]
pub type CHCCDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCCDS`"]
pub struct CHCCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCCDS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&self) -> CRBE_R {
        CRBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&self) -> CHCCDS_R {
        CHCCDS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W {
        TME_W { w: self }
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&mut self) -> CRBE_W {
        CRBE_W { w: self }
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&mut self) -> CHCCDS_W {
        CHCCDS_W { w: self }
    }
}
