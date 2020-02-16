#[doc = "Reader of register CHBRKCTR"]
pub type R = crate::R<u32, super::CHBRKCTR>;
#[doc = "Writer for register CHBRKCTR"]
pub type W = crate::W<u32, super::CHBRKCTR>;
#[doc = "Register CHBRKCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHBRKCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKE0`"]
pub type BKE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKE0`"]
pub struct BKE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BKE0_W<'a> {
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
#[doc = "Reader of field `BKP0`"]
pub type BKP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKP0`"]
pub struct BKP0_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP0_W<'a> {
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
#[doc = "Reader of field `CHMOE`"]
pub type CHMOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHMOE`"]
pub struct CHMOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOE_W<'a> {
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
#[doc = "Reader of field `CHAOE`"]
pub type CHAOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAOE`"]
pub struct CHAOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAOE_W<'a> {
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
#[doc = "Reader of field `BKF0`"]
pub type BKF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKF0`"]
pub struct BKF0_W<'a> {
    w: &'a mut W,
}
impl<'a> BKF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOCKLV`"]
pub type LOCKLV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCKLV`"]
pub struct LOCKLV_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `GFSEL0`"]
pub type GFSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFSEL0`"]
pub struct GFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> GFSEL0_W<'a> {
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
#[doc = "Reader of field `CHOSSI`"]
pub type CHOSSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHOSSI`"]
pub struct CHOSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOSSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CHOSSR`"]
pub type CHOSSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHOSSR`"]
pub struct CHOSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOSSR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CHDTG`"]
pub type CHDTG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHDTG`"]
pub struct CHDTG_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDTG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    pub fn bke0(&self) -> BKE0_R {
        BKE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    pub fn bkp0(&self) -> BKP0_R {
        BKP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    pub fn chmoe(&self) -> CHMOE_R {
        CHMOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    pub fn chaoe(&self) -> CHAOE_R {
        CHAOE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    pub fn bkf0(&self) -> BKF0_R {
        BKF0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&self) -> LOCKLV_R {
        LOCKLV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    pub fn gfsel0(&self) -> GFSEL0_R {
        GFSEL0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    pub fn chossi(&self) -> CHOSSI_R {
        CHOSSI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    pub fn chossr(&self) -> CHOSSR_R {
        CHOSSR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    pub fn chdtg(&self) -> CHDTG_R {
        CHDTG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    pub fn bke0(&mut self) -> BKE0_W {
        BKE0_W { w: self }
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    pub fn bkp0(&mut self) -> BKP0_W {
        BKP0_W { w: self }
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    pub fn chmoe(&mut self) -> CHMOE_W {
        CHMOE_W { w: self }
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    pub fn chaoe(&mut self) -> CHAOE_W {
        CHAOE_W { w: self }
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    pub fn bkf0(&mut self) -> BKF0_W {
        BKF0_W { w: self }
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&mut self) -> LOCKLV_W {
        LOCKLV_W { w: self }
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    pub fn gfsel0(&mut self) -> GFSEL0_W {
        GFSEL0_W { w: self }
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    pub fn chossi(&mut self) -> CHOSSI_W {
        CHOSSI_W { w: self }
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    pub fn chossr(&mut self) -> CHOSSR_W {
        CHOSSR_W { w: self }
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    pub fn chdtg(&mut self) -> CHDTG_W {
        CHDTG_W { w: self }
    }
}
