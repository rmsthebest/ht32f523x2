#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADISRS`"]
pub type ADISRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRS`"]
pub struct ADISRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRS_W<'a> {
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
#[doc = "Reader of field `ADISRG`"]
pub type ADISRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRG`"]
pub struct ADISRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRG_W<'a> {
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
#[doc = "Reader of field `ADISRC`"]
pub type ADISRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRC`"]
pub struct ADISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRC_W<'a> {
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
#[doc = "Reader of field `ADISRL`"]
pub type ADISRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRL`"]
pub struct ADISRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRL_W<'a> {
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
#[doc = "Reader of field `ADISRU`"]
pub type ADISRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRU`"]
pub struct ADISRU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRU_W<'a> {
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
#[doc = "Reader of field `ADISRO`"]
pub type ADISRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADISRO`"]
pub struct ADISRO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADISRO_W<'a> {
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
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    pub fn adisrs(&self) -> ADISRS_R {
        ADISRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    pub fn adisrg(&self) -> ADISRG_R {
        ADISRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    pub fn adisrc(&self) -> ADISRC_R {
        ADISRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    pub fn adisrl(&self) -> ADISRL_R {
        ADISRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    pub fn adisru(&self) -> ADISRU_R {
        ADISRU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    pub fn adisro(&self) -> ADISRO_R {
        ADISRO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    pub fn adisrs(&mut self) -> ADISRS_W {
        ADISRS_W { w: self }
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    pub fn adisrg(&mut self) -> ADISRG_W {
        ADISRG_W { w: self }
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    pub fn adisrc(&mut self) -> ADISRC_W {
        ADISRC_W { w: self }
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    pub fn adisrl(&mut self) -> ADISRL_W {
        ADISRL_W { w: self }
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    pub fn adisru(&mut self) -> ADISRU_W {
        ADISRU_W { w: self }
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    pub fn adisro(&mut self) -> ADISRO_W {
        ADISRO_W { w: self }
    }
}
