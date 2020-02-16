#[doc = "Reader of register CH4CR"]
pub type R = crate::R<u32, super::CH4CR>;
#[doc = "Writer for register CH4CR"]
pub type W = crate::W<u32, super::CH4CR>;
#[doc = "Register CH4CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH4CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
#[doc = "Reader of field `SWTRIG`"]
pub type SWTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG`"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
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
#[doc = "Reader of field `DWIDTH`"]
pub type DWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DWIDTH`"]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DSTAINC`"]
pub type DSTAINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSTAINC`"]
pub struct DSTAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTAINC_W<'a> {
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
#[doc = "Reader of field `DSTAMOD`"]
pub type DSTAMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSTAMOD`"]
pub struct DSTAMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTAMOD_W<'a> {
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
#[doc = "Reader of field `SRCAINC`"]
pub type SRCAINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRCAINC`"]
pub struct SRCAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCAINC_W<'a> {
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
#[doc = "Reader of field `SRCAMOD`"]
pub type SRCAMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRCAMOD`"]
pub struct SRCAMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCAMOD_W<'a> {
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
#[doc = "Reader of field `CHPRI`"]
pub type CHPRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHPRI`"]
pub struct CHPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIXAEN`"]
pub type FIXAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIXAEN`"]
pub struct FIXAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXAEN_W<'a> {
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
#[doc = "Reader of field `AUTORL`"]
pub type AUTORL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTORL`"]
pub struct AUTORL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    pub fn dstainc(&self) -> DSTAINC_R {
        DSTAINC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    pub fn dstamod(&self) -> DSTAMOD_R {
        DSTAMOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    pub fn srcainc(&self) -> SRCAINC_R {
        SRCAINC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    pub fn srcamod(&self) -> SRCAMOD_R {
        SRCAMOD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    pub fn fixaen(&self) -> FIXAEN_R {
        FIXAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    pub fn autorl(&self) -> AUTORL_R {
        AUTORL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    pub fn dstainc(&mut self) -> DSTAINC_W {
        DSTAINC_W { w: self }
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    pub fn dstamod(&mut self) -> DSTAMOD_W {
        DSTAMOD_W { w: self }
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    pub fn srcainc(&mut self) -> SRCAINC_W {
        SRCAINC_W { w: self }
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    pub fn srcamod(&mut self) -> SRCAMOD_W {
        SRCAMOD_W { w: self }
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    pub fn chpri(&mut self) -> CHPRI_W {
        CHPRI_W { w: self }
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    pub fn fixaen(&mut self) -> FIXAEN_W {
        FIXAEN_W { w: self }
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    pub fn autorl(&mut self) -> AUTORL_W {
        AUTORL_W { w: self }
    }
}
