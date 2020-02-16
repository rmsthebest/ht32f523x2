#[doc = "Reader of register PWRCU_BAKCR"]
pub type R = crate::R<u32, super::PWRCU_BAKCR>;
#[doc = "Writer for register PWRCU_BAKCR"]
pub type W = crate::W<u32, super::PWRCU_BAKCR>;
#[doc = "Register PWRCU_BAKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_BAKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAKRST`"]
pub type BAKRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAKRST`"]
pub struct BAKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BAKRST_W<'a> {
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
#[doc = "Reader of field `LDOLCM`"]
pub type LDOLCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDOLCM`"]
pub struct LDOLCM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOLCM_W<'a> {
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
#[doc = "Reader of field `LDOOFF`"]
pub type LDOOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDOOFF`"]
pub struct LDOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOOFF_W<'a> {
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
#[doc = "Reader of field `DMOSON`"]
pub type DMOSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMOSON`"]
pub struct DMOSON_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOSON_W<'a> {
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
#[doc = "Reader of field `WUPEN`"]
pub type WUPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN`"]
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
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
#[doc = "Reader of field `WUPIEN`"]
pub type WUPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPIEN`"]
pub struct WUPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPIEN_W<'a> {
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
#[doc = "Reader of field `V15RDYSC`"]
pub type V15RDYSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `V15RDYSC`"]
pub struct V15RDYSC_W<'a> {
    w: &'a mut W,
}
impl<'a> V15RDYSC_W<'a> {
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
#[doc = "Reader of field `DMOSSTS`"]
pub type DMOSSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMOSSTS`"]
pub struct DMOSSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOSSTS_W<'a> {
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
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    pub fn bakrst(&self) -> BAKRST_R {
        BAKRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LDOLCM"]
    #[inline(always)]
    pub fn ldolcm(&self) -> LDOLCM_R {
        LDOLCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    pub fn ldooff(&self) -> LDOOFF_R {
        LDOOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    pub fn dmoson(&self) -> DMOSON_R {
        DMOSON_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    pub fn wupien(&self) -> WUPIEN_R {
        WUPIEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    pub fn v15rdysc(&self) -> V15RDYSC_R {
        V15RDYSC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    pub fn dmossts(&self) -> DMOSSTS_R {
        DMOSSTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    pub fn bakrst(&mut self) -> BAKRST_W {
        BAKRST_W { w: self }
    }
    #[doc = "Bit 1 - LDOLCM"]
    #[inline(always)]
    pub fn ldolcm(&mut self) -> LDOLCM_W {
        LDOLCM_W { w: self }
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    pub fn ldooff(&mut self) -> LDOOFF_W {
        LDOOFF_W { w: self }
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    pub fn dmoson(&mut self) -> DMOSON_W {
        DMOSON_W { w: self }
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    pub fn wupien(&mut self) -> WUPIEN_W {
        WUPIEN_W { w: self }
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    pub fn v15rdysc(&mut self) -> V15RDYSC_W {
        V15RDYSC_W { w: self }
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    pub fn dmossts(&mut self) -> DMOSSTS_W {
        DMOSSTS_W { w: self }
    }
}
