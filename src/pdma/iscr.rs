#[doc = "Reader of register ISCR"]
pub type R = crate::R<u32, super::ISCR>;
#[doc = "Writer for register ISCR"]
pub type W = crate::W<u32, super::ISCR>;
#[doc = "Register ISCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEICLR0`"]
pub type GEICLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR0`"]
pub struct GEICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR0_W<'a> {
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
#[doc = "Reader of field `BEICLR0`"]
pub type BEICLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR0`"]
pub struct BEICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR0_W<'a> {
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
#[doc = "Reader of field `HTICLR0`"]
pub type HTICLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR0`"]
pub struct HTICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR0_W<'a> {
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
#[doc = "Reader of field `TCICLR0`"]
pub type TCICLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR0`"]
pub struct TCICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR0_W<'a> {
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
#[doc = "Reader of field `TEICLR0`"]
pub type TEICLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR0`"]
pub struct TEICLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR0_W<'a> {
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
#[doc = "Reader of field `GEICLR1`"]
pub type GEICLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR1`"]
pub struct GEICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR1_W<'a> {
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
#[doc = "Reader of field `BEICLR1`"]
pub type BEICLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR1`"]
pub struct BEICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR1_W<'a> {
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
#[doc = "Reader of field `HTICLR1`"]
pub type HTICLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR1`"]
pub struct HTICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR1_W<'a> {
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
#[doc = "Reader of field `TCICLR1`"]
pub type TCICLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR1`"]
pub struct TCICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR1_W<'a> {
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
#[doc = "Reader of field `TEICLR1`"]
pub type TEICLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR1`"]
pub struct TEICLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR1_W<'a> {
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
#[doc = "Reader of field `GEICLR2`"]
pub type GEICLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR2`"]
pub struct GEICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR2_W<'a> {
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
#[doc = "Reader of field `BEICLR2`"]
pub type BEICLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR2`"]
pub struct BEICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR2_W<'a> {
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
#[doc = "Reader of field `HTICLR2`"]
pub type HTICLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR2`"]
pub struct HTICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR2_W<'a> {
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
#[doc = "Reader of field `TCICLR2`"]
pub type TCICLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR2`"]
pub struct TCICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TEICLR2`"]
pub type TEICLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR2`"]
pub struct TEICLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR2_W<'a> {
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
#[doc = "Reader of field `GEICLR3`"]
pub type GEICLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR3`"]
pub struct GEICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR3_W<'a> {
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
#[doc = "Reader of field `BEICLR3`"]
pub type BEICLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR3`"]
pub struct BEICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR3_W<'a> {
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
#[doc = "Reader of field `HTICLR3`"]
pub type HTICLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR3`"]
pub struct HTICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR3_W<'a> {
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
#[doc = "Reader of field `TCICLR3`"]
pub type TCICLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR3`"]
pub struct TCICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR3_W<'a> {
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
#[doc = "Reader of field `TEICLR3`"]
pub type TEICLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR3`"]
pub struct TEICLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR3_W<'a> {
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
#[doc = "Reader of field `GEICLR4`"]
pub type GEICLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR4`"]
pub struct GEICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR4_W<'a> {
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
#[doc = "Reader of field `BEICLR4`"]
pub type BEICLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR4`"]
pub struct BEICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR4_W<'a> {
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
#[doc = "Reader of field `HTICLR4`"]
pub type HTICLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR4`"]
pub struct HTICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TCICLR4`"]
pub type TCICLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR4`"]
pub struct TCICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TEICLR4`"]
pub type TEICLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR4`"]
pub struct TEICLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR4_W<'a> {
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
#[doc = "Reader of field `GEICLR5`"]
pub type GEICLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR5`"]
pub struct GEICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR5_W<'a> {
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
#[doc = "Reader of field `BEICLR5`"]
pub type BEICLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR5`"]
pub struct BEICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR5_W<'a> {
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
#[doc = "Reader of field `HTICLR5`"]
pub type HTICLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR5`"]
pub struct HTICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TCICLR5`"]
pub type TCICLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR5`"]
pub struct TCICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TEICLR5`"]
pub type TEICLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR5`"]
pub struct TEICLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GEICLR0"]
    #[inline(always)]
    pub fn geiclr0(&self) -> GEICLR0_R {
        GEICLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEICLR0"]
    #[inline(always)]
    pub fn beiclr0(&self) -> BEICLR0_R {
        BEICLR0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTICLR0"]
    #[inline(always)]
    pub fn hticlr0(&self) -> HTICLR0_R {
        HTICLR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCICLR0"]
    #[inline(always)]
    pub fn tciclr0(&self) -> TCICLR0_R {
        TCICLR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEICLR0"]
    #[inline(always)]
    pub fn teiclr0(&self) -> TEICLR0_R {
        TEICLR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEICLR1"]
    #[inline(always)]
    pub fn geiclr1(&self) -> GEICLR1_R {
        GEICLR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEICLR1"]
    #[inline(always)]
    pub fn beiclr1(&self) -> BEICLR1_R {
        BEICLR1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTICLR1"]
    #[inline(always)]
    pub fn hticlr1(&self) -> HTICLR1_R {
        HTICLR1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCICLR1"]
    #[inline(always)]
    pub fn tciclr1(&self) -> TCICLR1_R {
        TCICLR1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEICLR1"]
    #[inline(always)]
    pub fn teiclr1(&self) -> TEICLR1_R {
        TEICLR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEICLR2"]
    #[inline(always)]
    pub fn geiclr2(&self) -> GEICLR2_R {
        GEICLR2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEICLR2"]
    #[inline(always)]
    pub fn beiclr2(&self) -> BEICLR2_R {
        BEICLR2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTICLR2"]
    #[inline(always)]
    pub fn hticlr2(&self) -> HTICLR2_R {
        HTICLR2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCICLR2"]
    #[inline(always)]
    pub fn tciclr2(&self) -> TCICLR2_R {
        TCICLR2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEICLR2"]
    #[inline(always)]
    pub fn teiclr2(&self) -> TEICLR2_R {
        TEICLR2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEICLR3"]
    #[inline(always)]
    pub fn geiclr3(&self) -> GEICLR3_R {
        GEICLR3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEICLR3"]
    #[inline(always)]
    pub fn beiclr3(&self) -> BEICLR3_R {
        BEICLR3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTICLR3"]
    #[inline(always)]
    pub fn hticlr3(&self) -> HTICLR3_R {
        HTICLR3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCICLR3"]
    #[inline(always)]
    pub fn tciclr3(&self) -> TCICLR3_R {
        TCICLR3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEICLR3"]
    #[inline(always)]
    pub fn teiclr3(&self) -> TEICLR3_R {
        TEICLR3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEICLR4"]
    #[inline(always)]
    pub fn geiclr4(&self) -> GEICLR4_R {
        GEICLR4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEICLR4"]
    #[inline(always)]
    pub fn beiclr4(&self) -> BEICLR4_R {
        BEICLR4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTICLR4"]
    #[inline(always)]
    pub fn hticlr4(&self) -> HTICLR4_R {
        HTICLR4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCICLR4"]
    #[inline(always)]
    pub fn tciclr4(&self) -> TCICLR4_R {
        TCICLR4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEICLR4"]
    #[inline(always)]
    pub fn teiclr4(&self) -> TEICLR4_R {
        TEICLR4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEICLR5"]
    #[inline(always)]
    pub fn geiclr5(&self) -> GEICLR5_R {
        GEICLR5_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEICLR5"]
    #[inline(always)]
    pub fn beiclr5(&self) -> BEICLR5_R {
        BEICLR5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTICLR5"]
    #[inline(always)]
    pub fn hticlr5(&self) -> HTICLR5_R {
        HTICLR5_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCICLR5"]
    #[inline(always)]
    pub fn tciclr5(&self) -> TCICLR5_R {
        TCICLR5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEICLR5"]
    #[inline(always)]
    pub fn teiclr5(&self) -> TEICLR5_R {
        TEICLR5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEICLR0"]
    #[inline(always)]
    pub fn geiclr0(&mut self) -> GEICLR0_W {
        GEICLR0_W { w: self }
    }
    #[doc = "Bit 1 - BEICLR0"]
    #[inline(always)]
    pub fn beiclr0(&mut self) -> BEICLR0_W {
        BEICLR0_W { w: self }
    }
    #[doc = "Bit 2 - HTICLR0"]
    #[inline(always)]
    pub fn hticlr0(&mut self) -> HTICLR0_W {
        HTICLR0_W { w: self }
    }
    #[doc = "Bit 3 - TCICLR0"]
    #[inline(always)]
    pub fn tciclr0(&mut self) -> TCICLR0_W {
        TCICLR0_W { w: self }
    }
    #[doc = "Bit 4 - TEICLR0"]
    #[inline(always)]
    pub fn teiclr0(&mut self) -> TEICLR0_W {
        TEICLR0_W { w: self }
    }
    #[doc = "Bit 5 - GEICLR1"]
    #[inline(always)]
    pub fn geiclr1(&mut self) -> GEICLR1_W {
        GEICLR1_W { w: self }
    }
    #[doc = "Bit 6 - BEICLR1"]
    #[inline(always)]
    pub fn beiclr1(&mut self) -> BEICLR1_W {
        BEICLR1_W { w: self }
    }
    #[doc = "Bit 7 - HTICLR1"]
    #[inline(always)]
    pub fn hticlr1(&mut self) -> HTICLR1_W {
        HTICLR1_W { w: self }
    }
    #[doc = "Bit 8 - TCICLR1"]
    #[inline(always)]
    pub fn tciclr1(&mut self) -> TCICLR1_W {
        TCICLR1_W { w: self }
    }
    #[doc = "Bit 9 - TEICLR1"]
    #[inline(always)]
    pub fn teiclr1(&mut self) -> TEICLR1_W {
        TEICLR1_W { w: self }
    }
    #[doc = "Bit 10 - GEICLR2"]
    #[inline(always)]
    pub fn geiclr2(&mut self) -> GEICLR2_W {
        GEICLR2_W { w: self }
    }
    #[doc = "Bit 11 - BEICLR2"]
    #[inline(always)]
    pub fn beiclr2(&mut self) -> BEICLR2_W {
        BEICLR2_W { w: self }
    }
    #[doc = "Bit 12 - HTICLR2"]
    #[inline(always)]
    pub fn hticlr2(&mut self) -> HTICLR2_W {
        HTICLR2_W { w: self }
    }
    #[doc = "Bit 13 - TCICLR2"]
    #[inline(always)]
    pub fn tciclr2(&mut self) -> TCICLR2_W {
        TCICLR2_W { w: self }
    }
    #[doc = "Bit 14 - TEICLR2"]
    #[inline(always)]
    pub fn teiclr2(&mut self) -> TEICLR2_W {
        TEICLR2_W { w: self }
    }
    #[doc = "Bit 15 - GEICLR3"]
    #[inline(always)]
    pub fn geiclr3(&mut self) -> GEICLR3_W {
        GEICLR3_W { w: self }
    }
    #[doc = "Bit 16 - BEICLR3"]
    #[inline(always)]
    pub fn beiclr3(&mut self) -> BEICLR3_W {
        BEICLR3_W { w: self }
    }
    #[doc = "Bit 17 - HTICLR3"]
    #[inline(always)]
    pub fn hticlr3(&mut self) -> HTICLR3_W {
        HTICLR3_W { w: self }
    }
    #[doc = "Bit 18 - TCICLR3"]
    #[inline(always)]
    pub fn tciclr3(&mut self) -> TCICLR3_W {
        TCICLR3_W { w: self }
    }
    #[doc = "Bit 19 - TEICLR3"]
    #[inline(always)]
    pub fn teiclr3(&mut self) -> TEICLR3_W {
        TEICLR3_W { w: self }
    }
    #[doc = "Bit 20 - GEICLR4"]
    #[inline(always)]
    pub fn geiclr4(&mut self) -> GEICLR4_W {
        GEICLR4_W { w: self }
    }
    #[doc = "Bit 21 - BEICLR4"]
    #[inline(always)]
    pub fn beiclr4(&mut self) -> BEICLR4_W {
        BEICLR4_W { w: self }
    }
    #[doc = "Bit 22 - HTICLR4"]
    #[inline(always)]
    pub fn hticlr4(&mut self) -> HTICLR4_W {
        HTICLR4_W { w: self }
    }
    #[doc = "Bit 23 - TCICLR4"]
    #[inline(always)]
    pub fn tciclr4(&mut self) -> TCICLR4_W {
        TCICLR4_W { w: self }
    }
    #[doc = "Bit 24 - TEICLR4"]
    #[inline(always)]
    pub fn teiclr4(&mut self) -> TEICLR4_W {
        TEICLR4_W { w: self }
    }
    #[doc = "Bit 25 - GEICLR5"]
    #[inline(always)]
    pub fn geiclr5(&mut self) -> GEICLR5_W {
        GEICLR5_W { w: self }
    }
    #[doc = "Bit 26 - BEICLR5"]
    #[inline(always)]
    pub fn beiclr5(&mut self) -> BEICLR5_W {
        BEICLR5_W { w: self }
    }
    #[doc = "Bit 27 - HTICLR5"]
    #[inline(always)]
    pub fn hticlr5(&mut self) -> HTICLR5_W {
        HTICLR5_W { w: self }
    }
    #[doc = "Bit 28 - TCICLR5"]
    #[inline(always)]
    pub fn tciclr5(&mut self) -> TCICLR5_W {
        TCICLR5_W { w: self }
    }
    #[doc = "Bit 29 - TEICLR5"]
    #[inline(always)]
    pub fn teiclr5(&mut self) -> TEICLR5_W {
        TEICLR5_W { w: self }
    }
}
