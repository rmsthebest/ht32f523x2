#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
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
#[doc = "Reader of field `RTCSRC`"]
pub type RTCSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCSRC`"]
pub struct RTCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSRC_W<'a> {
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
#[doc = "Reader of field `LSIEN`"]
pub type LSIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSIEN`"]
pub struct LSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIEN_W<'a> {
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
#[doc = "Reader of field `LSEEN`"]
pub type LSEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSEEN`"]
pub struct LSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEEN_W<'a> {
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
#[doc = "Reader of field `CMPCLR`"]
pub type CMPCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPCLR`"]
pub struct CMPCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCLR_W<'a> {
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
#[doc = "Reader of field `LSESM`"]
pub type LSESM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSESM`"]
pub struct LSESM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESM_W<'a> {
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
#[doc = "Reader of field `RPRE`"]
pub type RPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPRE`"]
pub struct RPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ROEN`"]
pub type ROEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROEN`"]
pub struct ROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROEN_W<'a> {
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
#[doc = "Reader of field `ROES`"]
pub type ROES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROES`"]
pub struct ROES_W<'a> {
    w: &'a mut W,
}
impl<'a> ROES_W<'a> {
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
#[doc = "Reader of field `ROWM`"]
pub type ROWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROWM`"]
pub struct ROWM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROWM_W<'a> {
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
#[doc = "Reader of field `ROAP`"]
pub type ROAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROAP`"]
pub struct ROAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ROAP_W<'a> {
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
#[doc = "Reader of field `ROLF`"]
pub type ROLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROLF`"]
pub struct ROLF_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LSEEN_R {
        LSEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    pub fn cmpclr(&self) -> CMPCLR_R {
        CMPCLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    pub fn lsesm(&self) -> LSESM_R {
        LSESM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    pub fn rpre(&self) -> RPRE_R {
        RPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    pub fn roen(&self) -> ROEN_R {
        ROEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    pub fn roes(&self) -> ROES_R {
        ROES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    pub fn rowm(&self) -> ROWM_R {
        ROWM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    pub fn roap(&self) -> ROAP_R {
        ROAP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    pub fn rolf(&self) -> ROLF_R {
        ROLF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RTCSRC_W {
        RTCSRC_W { w: self }
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&mut self) -> LSIEN_W {
        LSIEN_W { w: self }
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&mut self) -> LSEEN_W {
        LSEEN_W { w: self }
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    pub fn cmpclr(&mut self) -> CMPCLR_W {
        CMPCLR_W { w: self }
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    pub fn lsesm(&mut self) -> LSESM_W {
        LSESM_W { w: self }
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    pub fn rpre(&mut self) -> RPRE_W {
        RPRE_W { w: self }
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    pub fn roen(&mut self) -> ROEN_W {
        ROEN_W { w: self }
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    pub fn roes(&mut self) -> ROES_W {
        ROES_W { w: self }
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    pub fn rowm(&mut self) -> ROWM_W {
        ROWM_W { w: self }
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    pub fn roap(&mut self) -> ROAP_W {
        ROAP_W { w: self }
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    pub fn rolf(&mut self) -> ROLF_W {
        ROLF_W { w: self }
    }
}
