#[doc = "Reader of register CFCR"]
pub type R = crate::R<u32, super::CFCR>;
#[doc = "Writer for register CFCR"]
pub type W = crate::W<u32, super::CFCR>;
#[doc = "Register CFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAIT`"]
pub type WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT`"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PFBE`"]
pub type PFBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFBE`"]
pub struct PFBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFBE_W<'a> {
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
#[doc = "Reader of field `DCDB`"]
pub type DCDB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDB`"]
pub struct DCDB_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDB_W<'a> {
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
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE`"]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    pub fn pfbe(&self) -> PFBE_R {
        PFBE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    pub fn dcdb(&self) -> DCDB_R {
        DCDB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    pub fn pfbe(&mut self) -> PFBE_W {
        PFBE_W { w: self }
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    pub fn dcdb(&mut self) -> DCDB_W {
        DCDB_W { w: self }
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
}
