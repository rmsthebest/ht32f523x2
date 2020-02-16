#[doc = "Reader of register DRVR"]
pub type R = crate::R<u32, super::DRVR>;
#[doc = "Writer for register DRVR"]
pub type W = crate::W<u32, super::DRVR>;
#[doc = "Register DRVR `reset()`'s with value 0"]
impl crate::ResetValue for super::DRVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DV0`"]
pub type DV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV0`"]
pub struct DV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DV0_W<'a> {
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
#[doc = "Reader of field `DV1`"]
pub type DV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV1`"]
pub struct DV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DV1_W<'a> {
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
#[doc = "Reader of field `DV2`"]
pub type DV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV2`"]
pub struct DV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DV2_W<'a> {
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
#[doc = "Reader of field `DV3`"]
pub type DV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV3`"]
pub struct DV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DV3_W<'a> {
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
#[doc = "Reader of field `DV4`"]
pub type DV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV4`"]
pub struct DV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DV4_W<'a> {
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
#[doc = "Reader of field `DV5`"]
pub type DV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV5`"]
pub struct DV5_W<'a> {
    w: &'a mut W,
}
impl<'a> DV5_W<'a> {
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
#[doc = "Reader of field `DV6`"]
pub type DV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV6`"]
pub struct DV6_W<'a> {
    w: &'a mut W,
}
impl<'a> DV6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DV7`"]
pub type DV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV7`"]
pub struct DV7_W<'a> {
    w: &'a mut W,
}
impl<'a> DV7_W<'a> {
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
#[doc = "Reader of field `DV8`"]
pub type DV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV8`"]
pub struct DV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DV8_W<'a> {
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
#[doc = "Reader of field `DV9`"]
pub type DV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV9`"]
pub struct DV9_W<'a> {
    w: &'a mut W,
}
impl<'a> DV9_W<'a> {
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
#[doc = "Reader of field `DV10`"]
pub type DV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV10`"]
pub struct DV10_W<'a> {
    w: &'a mut W,
}
impl<'a> DV10_W<'a> {
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
#[doc = "Reader of field `DV11`"]
pub type DV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV11`"]
pub struct DV11_W<'a> {
    w: &'a mut W,
}
impl<'a> DV11_W<'a> {
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
#[doc = "Reader of field `DV14`"]
pub type DV14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV14`"]
pub struct DV14_W<'a> {
    w: &'a mut W,
}
impl<'a> DV14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DV15`"]
pub type DV15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV15`"]
pub struct DV15_W<'a> {
    w: &'a mut W,
}
impl<'a> DV15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> DV0_R {
        DV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> DV1_R {
        DV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> DV2_R {
        DV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    pub fn dv3(&self) -> DV3_R {
        DV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    pub fn dv4(&self) -> DV4_R {
        DV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&self) -> DV5_R {
        DV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> DV6_R {
        DV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&self) -> DV7_R {
        DV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&self) -> DV8_R {
        DV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&self) -> DV9_R {
        DV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&self) -> DV10_R {
        DV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&self) -> DV11_R {
        DV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    pub fn dv14(&self) -> DV14_R {
        DV14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    pub fn dv15(&self) -> DV15_R {
        DV15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    pub fn dv0(&mut self) -> DV0_W {
        DV0_W { w: self }
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    pub fn dv1(&mut self) -> DV1_W {
        DV1_W { w: self }
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    pub fn dv2(&mut self) -> DV2_W {
        DV2_W { w: self }
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    pub fn dv3(&mut self) -> DV3_W {
        DV3_W { w: self }
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    pub fn dv4(&mut self) -> DV4_W {
        DV4_W { w: self }
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&mut self) -> DV5_W {
        DV5_W { w: self }
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&mut self) -> DV6_W {
        DV6_W { w: self }
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&mut self) -> DV7_W {
        DV7_W { w: self }
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&mut self) -> DV8_W {
        DV8_W { w: self }
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&mut self) -> DV9_W {
        DV9_W { w: self }
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&mut self) -> DV10_W {
        DV10_W { w: self }
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&mut self) -> DV11_W {
        DV11_W { w: self }
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    pub fn dv14(&mut self) -> DV14_W {
        DV14_W { w: self }
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    pub fn dv15(&mut self) -> DV15_W {
        DV15_W { w: self }
    }
}
