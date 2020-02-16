#[doc = "Reader of register EP5CSR"]
pub type R = crate::R<u32, super::EP5CSR>;
#[doc = "Writer for register EP5CSR"]
pub type W = crate::W<u32, super::EP5CSR>;
#[doc = "Register EP5CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP5CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTGTX`"]
pub type DTGTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTGTX`"]
pub struct DTGTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGTX_W<'a> {
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
#[doc = "Reader of field `NAKTX`"]
pub type NAKTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKTX`"]
pub struct NAKTX_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKTX_W<'a> {
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
#[doc = "Reader of field `STLTX`"]
pub type STLTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STLTX`"]
pub struct STLTX_W<'a> {
    w: &'a mut W,
}
impl<'a> STLTX_W<'a> {
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
#[doc = "Reader of field `DTGRX`"]
pub type DTGRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTGRX`"]
pub struct DTGRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGRX_W<'a> {
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
#[doc = "Reader of field `NAKRX`"]
pub type NAKRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKRX`"]
pub struct NAKRX_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKRX_W<'a> {
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
#[doc = "Reader of field `STLRX`"]
pub type STLRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STLRX`"]
pub struct STLRX_W<'a> {
    w: &'a mut W,
}
impl<'a> STLRX_W<'a> {
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
#[doc = "Reader of field `MDBTG`"]
pub type MDBTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDBTG`"]
pub struct MDBTG_W<'a> {
    w: &'a mut W,
}
impl<'a> MDBTG_W<'a> {
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
#[doc = "Reader of field `UDBTG`"]
pub type UDBTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDBTG`"]
pub struct UDBTG_W<'a> {
    w: &'a mut W,
}
impl<'a> UDBTG_W<'a> {
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
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&self) -> DTGTX_R {
        DTGTX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&self) -> NAKTX_R {
        NAKTX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&self) -> STLTX_R {
        STLTX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&self) -> DTGRX_R {
        DTGRX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&self) -> NAKRX_R {
        NAKRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&self) -> STLRX_R {
        STLRX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    pub fn mdbtg(&self) -> MDBTG_R {
        MDBTG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    pub fn udbtg(&self) -> UDBTG_R {
        UDBTG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&mut self) -> DTGTX_W {
        DTGTX_W { w: self }
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&mut self) -> NAKTX_W {
        NAKTX_W { w: self }
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&mut self) -> STLTX_W {
        STLTX_W { w: self }
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&mut self) -> DTGRX_W {
        DTGRX_W { w: self }
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&mut self) -> NAKRX_W {
        NAKRX_W { w: self }
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&mut self) -> STLRX_W {
        STLRX_W { w: self }
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    pub fn mdbtg(&mut self) -> MDBTG_W {
        MDBTG_W { w: self }
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    pub fn udbtg(&mut self) -> UDBTG_W {
        UDBTG_W { w: self }
    }
}
