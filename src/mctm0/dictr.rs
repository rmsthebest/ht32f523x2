#[doc = "Reader of register DICTR"]
pub type R = crate::R<u32, super::DICTR>;
#[doc = "Writer for register DICTR"]
pub type W = crate::W<u32, super::DICTR>;
#[doc = "Register DICTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DICTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CCIE`"]
pub type CH0CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CCIE`"]
pub struct CH0CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCIE_W<'a> {
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
#[doc = "Reader of field `CH1CCIE`"]
pub type CH1CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CCIE`"]
pub struct CH1CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCIE_W<'a> {
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
#[doc = "Reader of field `CH2CCIE`"]
pub type CH2CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2CCIE`"]
pub struct CH2CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCIE_W<'a> {
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
#[doc = "Reader of field `CH3CCIE`"]
pub type CH3CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3CCIE`"]
pub struct CH3CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCIE_W<'a> {
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
#[doc = "Reader of field `UEV1IE`"]
pub type UEV1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEV1IE`"]
pub struct UEV1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEV1IE_W<'a> {
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
#[doc = "Reader of field `UEV2IE`"]
pub type UEV2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEV2IE`"]
pub struct UEV2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEV2IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TEVIE`"]
pub type TEVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEVIE`"]
pub struct TEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BRKIE`"]
pub type BRKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRKIE`"]
pub struct BRKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CH0CCDE`"]
pub type CH0CCDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CCDE`"]
pub struct CH0CCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCDE_W<'a> {
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
#[doc = "Reader of field `CH1CCDE`"]
pub type CH1CCDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CCDE`"]
pub struct CH1CCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCDE_W<'a> {
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
#[doc = "Reader of field `CH2CCDE`"]
pub type CH2CCDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2CCDE`"]
pub struct CH2CCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CH3CCDE`"]
pub type CH3CCDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3CCDE`"]
pub struct CH3CCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `UEV1DE`"]
pub type UEV1DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEV1DE`"]
pub struct UEV1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEV1DE_W<'a> {
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
#[doc = "Reader of field `UEV2DE`"]
pub type UEV2DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEV2DE`"]
pub struct UEV2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEV2DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TEVDE`"]
pub type TEVDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEVDE`"]
pub struct TEVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEVDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> CH0CCIE_R {
        CH0CCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> CH1CCIE_R {
        CH1CCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&self) -> CH2CCIE_R {
        CH2CCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&self) -> CH3CCIE_R {
        CH3CCIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    pub fn uev1ie(&self) -> UEV1IE_R {
        UEV1IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    pub fn uev2ie(&self) -> UEV2IE_R {
        UEV2IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TEVIE_R {
        TEVIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&self) -> CH0CCDE_R {
        CH0CCDE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&self) -> CH1CCDE_R {
        CH1CCDE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    pub fn ch2ccde(&self) -> CH2CCDE_R {
        CH2CCDE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    pub fn ch3ccde(&self) -> CH3CCDE_R {
        CH3CCDE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    pub fn uev1de(&self) -> UEV1DE_R {
        UEV1DE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    pub fn uev2de(&self) -> UEV2DE_R {
        UEV2DE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    pub fn tevde(&self) -> TEVDE_R {
        TEVDE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&mut self) -> CH0CCIE_W {
        CH0CCIE_W { w: self }
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&mut self) -> CH1CCIE_W {
        CH1CCIE_W { w: self }
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&mut self) -> CH2CCIE_W {
        CH2CCIE_W { w: self }
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&mut self) -> CH3CCIE_W {
        CH3CCIE_W { w: self }
    }
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    pub fn uev1ie(&mut self) -> UEV1IE_W {
        UEV1IE_W { w: self }
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    pub fn uev2ie(&mut self) -> UEV2IE_W {
        UEV2IE_W { w: self }
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&mut self) -> TEVIE_W {
        TEVIE_W { w: self }
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    pub fn brkie(&mut self) -> BRKIE_W {
        BRKIE_W { w: self }
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&mut self) -> CH0CCDE_W {
        CH0CCDE_W { w: self }
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&mut self) -> CH1CCDE_W {
        CH1CCDE_W { w: self }
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    pub fn ch2ccde(&mut self) -> CH2CCDE_W {
        CH2CCDE_W { w: self }
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    pub fn ch3ccde(&mut self) -> CH3CCDE_W {
        CH3CCDE_W { w: self }
    }
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    pub fn uev1de(&mut self) -> UEV1DE_W {
        UEV1DE_W { w: self }
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    pub fn uev2de(&mut self) -> UEV2DE_W {
        UEV2DE_W { w: self }
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    pub fn tevde(&mut self) -> TEVDE_W {
        TEVDE_W { w: self }
    }
}
