#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXFTL`"]
pub type TXFTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFTL`"]
pub struct TXFTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTL_W<'a> {
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
#[doc = "Reader of field `TXFUD`"]
pub type TXFUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFUD`"]
pub struct TXFUD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFUD_W<'a> {
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
#[doc = "Reader of field `TXFOV`"]
pub type TXFOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFOV`"]
pub struct TXFOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFOV_W<'a> {
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
#[doc = "Reader of field `TXFEMT`"]
pub type TXFEMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFEMT`"]
pub struct TXFEMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEMT_W<'a> {
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
#[doc = "Reader of field `TXFFUL`"]
pub type TXFFUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFFUL`"]
pub struct TXFFUL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFUL_W<'a> {
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
#[doc = "Reader of field `RXFTL`"]
pub type RXFTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFTL`"]
pub struct RXFTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTL_W<'a> {
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
#[doc = "Reader of field `RXFUD`"]
pub type RXFUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFUD`"]
pub struct RXFUD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFUD_W<'a> {
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
#[doc = "Reader of field `RXFOV`"]
pub type RXFOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFOV`"]
pub struct RXFOV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOV_W<'a> {
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
#[doc = "Reader of field `RXFEMT`"]
pub type RXFEMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFEMT`"]
pub struct RXFEMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFEMT_W<'a> {
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
#[doc = "Reader of field `RXFFUL`"]
pub type RXFFUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFUL`"]
pub struct RXFFUL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CHS`"]
pub type CHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHS`"]
pub struct CHS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHS_W<'a> {
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
#[doc = "Reader of field `TXBUSY`"]
pub type TXBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBUSY`"]
pub struct TXBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUSY_W<'a> {
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
#[doc = "Reader of field `CLKRDY`"]
pub type CLKRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKRDY`"]
pub struct CLKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKRDY_W<'a> {
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
#[doc = "Reader of field `TXFS`"]
pub type TXFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFS`"]
pub struct TXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RXFS`"]
pub type RXFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFS`"]
pub struct RXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    pub fn txftl(&self) -> TXFTL_R {
        TXFTL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    pub fn txfud(&self) -> TXFUD_R {
        TXFUD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    pub fn txfov(&self) -> TXFOV_R {
        TXFOV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    pub fn txfemt(&self) -> TXFEMT_R {
        TXFEMT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    pub fn txfful(&self) -> TXFFUL_R {
        TXFFUL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    pub fn rxftl(&self) -> RXFTL_R {
        RXFTL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    pub fn rxfud(&self) -> RXFUD_R {
        RXFUD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    pub fn rxfov(&self) -> RXFOV_R {
        RXFOV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    pub fn rxfemt(&self) -> RXFEMT_R {
        RXFEMT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    pub fn rxfful(&self) -> RXFFUL_R {
        RXFFUL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXFTL"]
    #[inline(always)]
    pub fn txftl(&mut self) -> TXFTL_W {
        TXFTL_W { w: self }
    }
    #[doc = "Bit 1 - TXFUD"]
    #[inline(always)]
    pub fn txfud(&mut self) -> TXFUD_W {
        TXFUD_W { w: self }
    }
    #[doc = "Bit 2 - TXFOV"]
    #[inline(always)]
    pub fn txfov(&mut self) -> TXFOV_W {
        TXFOV_W { w: self }
    }
    #[doc = "Bit 3 - TXFEMT"]
    #[inline(always)]
    pub fn txfemt(&mut self) -> TXFEMT_W {
        TXFEMT_W { w: self }
    }
    #[doc = "Bit 4 - TXFFUL"]
    #[inline(always)]
    pub fn txfful(&mut self) -> TXFFUL_W {
        TXFFUL_W { w: self }
    }
    #[doc = "Bit 8 - RXFTL"]
    #[inline(always)]
    pub fn rxftl(&mut self) -> RXFTL_W {
        RXFTL_W { w: self }
    }
    #[doc = "Bit 9 - RXFUD"]
    #[inline(always)]
    pub fn rxfud(&mut self) -> RXFUD_W {
        RXFUD_W { w: self }
    }
    #[doc = "Bit 10 - RXFOV"]
    #[inline(always)]
    pub fn rxfov(&mut self) -> RXFOV_W {
        RXFOV_W { w: self }
    }
    #[doc = "Bit 11 - RXFEMT"]
    #[inline(always)]
    pub fn rxfemt(&mut self) -> RXFEMT_W {
        RXFEMT_W { w: self }
    }
    #[doc = "Bit 12 - RXFFUL"]
    #[inline(always)]
    pub fn rxfful(&mut self) -> RXFFUL_W {
        RXFFUL_W { w: self }
    }
    #[doc = "Bit 16 - CHS"]
    #[inline(always)]
    pub fn chs(&mut self) -> CHS_W {
        CHS_W { w: self }
    }
    #[doc = "Bit 17 - TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&mut self) -> TXBUSY_W {
        TXBUSY_W { w: self }
    }
    #[doc = "Bit 18 - CLKRDY"]
    #[inline(always)]
    pub fn clkrdy(&mut self) -> CLKRDY_W {
        CLKRDY_W { w: self }
    }
    #[doc = "Bits 24:27 - TXFS"]
    #[inline(always)]
    pub fn txfs(&mut self) -> TXFS_W {
        TXFS_W { w: self }
    }
    #[doc = "Bits 28:31 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&mut self) -> RXFS_W {
        RXFS_W { w: self }
    }
}
