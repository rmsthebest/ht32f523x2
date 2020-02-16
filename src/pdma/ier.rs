#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEIE0`"]
pub type GEIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE0`"]
pub struct GEIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE0_W<'a> {
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
#[doc = "Reader of field `BEIE0`"]
pub type BEIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE0`"]
pub struct BEIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE0_W<'a> {
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
#[doc = "Reader of field `HTIE0`"]
pub type HTIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE0`"]
pub struct HTIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE0_W<'a> {
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
#[doc = "Reader of field `TCIE0`"]
pub type TCIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE0`"]
pub struct TCIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE0_W<'a> {
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
#[doc = "Reader of field `TEIE0`"]
pub type TEIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE0`"]
pub struct TEIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE0_W<'a> {
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
#[doc = "Reader of field `GEIE1`"]
pub type GEIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE1`"]
pub struct GEIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE1_W<'a> {
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
#[doc = "Reader of field `BEIE1`"]
pub type BEIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE1`"]
pub struct BEIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE1_W<'a> {
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
#[doc = "Reader of field `HTIE1`"]
pub type HTIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE1`"]
pub struct HTIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE1_W<'a> {
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
#[doc = "Reader of field `TCIE1`"]
pub type TCIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE1`"]
pub struct TCIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE1_W<'a> {
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
#[doc = "Reader of field `TEIE1`"]
pub type TEIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE1`"]
pub struct TEIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE1_W<'a> {
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
#[doc = "Reader of field `GEIE2`"]
pub type GEIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE2`"]
pub struct GEIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE2_W<'a> {
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
#[doc = "Reader of field `BEIE2`"]
pub type BEIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE2`"]
pub struct BEIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE2_W<'a> {
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
#[doc = "Reader of field `HTIE2`"]
pub type HTIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE2`"]
pub struct HTIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE2_W<'a> {
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
#[doc = "Reader of field `TCIE2`"]
pub type TCIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE2`"]
pub struct TCIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE2_W<'a> {
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
#[doc = "Reader of field `TEIE2`"]
pub type TEIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE2`"]
pub struct TEIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE2_W<'a> {
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
#[doc = "Reader of field `GEIE3`"]
pub type GEIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE3`"]
pub struct GEIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE3_W<'a> {
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
#[doc = "Reader of field `BEIE3`"]
pub type BEIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE3`"]
pub struct BEIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE3_W<'a> {
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
#[doc = "Reader of field `HTIE3`"]
pub type HTIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE3`"]
pub struct HTIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE3_W<'a> {
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
#[doc = "Reader of field `TCIE3`"]
pub type TCIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE3`"]
pub struct TCIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE3_W<'a> {
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
#[doc = "Reader of field `TEIE3`"]
pub type TEIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE3`"]
pub struct TEIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE3_W<'a> {
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
#[doc = "Reader of field `GEIE4`"]
pub type GEIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE4`"]
pub struct GEIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE4_W<'a> {
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
#[doc = "Reader of field `BEIE4`"]
pub type BEIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE4`"]
pub struct BEIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE4_W<'a> {
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
#[doc = "Reader of field `HTIE4`"]
pub type HTIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE4`"]
pub struct HTIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE4_W<'a> {
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
#[doc = "Reader of field `TCIE4`"]
pub type TCIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE4`"]
pub struct TCIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE4_W<'a> {
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
#[doc = "Reader of field `TEIE4`"]
pub type TEIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE4`"]
pub struct TEIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE4_W<'a> {
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
#[doc = "Reader of field `GEIE5`"]
pub type GEIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE5`"]
pub struct GEIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE5_W<'a> {
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
#[doc = "Reader of field `BEIE5`"]
pub type BEIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE5`"]
pub struct BEIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE5_W<'a> {
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
#[doc = "Reader of field `HTIE5`"]
pub type HTIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE5`"]
pub struct HTIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE5_W<'a> {
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
#[doc = "Reader of field `TCIE5`"]
pub type TCIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE5`"]
pub struct TCIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE5_W<'a> {
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
#[doc = "Reader of field `TEIE5`"]
pub type TEIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE5`"]
pub struct TEIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE5_W<'a> {
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
    #[doc = "Bit 0 - GEIE0"]
    #[inline(always)]
    pub fn geie0(&self) -> GEIE0_R {
        GEIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEIE0"]
    #[inline(always)]
    pub fn beie0(&self) -> BEIE0_R {
        BEIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTIE0"]
    #[inline(always)]
    pub fn htie0(&self) -> HTIE0_R {
        HTIE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCIE0"]
    #[inline(always)]
    pub fn tcie0(&self) -> TCIE0_R {
        TCIE0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEIE0"]
    #[inline(always)]
    pub fn teie0(&self) -> TEIE0_R {
        TEIE0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEIE1"]
    #[inline(always)]
    pub fn geie1(&self) -> GEIE1_R {
        GEIE1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEIE1"]
    #[inline(always)]
    pub fn beie1(&self) -> BEIE1_R {
        BEIE1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTIE1"]
    #[inline(always)]
    pub fn htie1(&self) -> HTIE1_R {
        HTIE1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCIE1"]
    #[inline(always)]
    pub fn tcie1(&self) -> TCIE1_R {
        TCIE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEIE1"]
    #[inline(always)]
    pub fn teie1(&self) -> TEIE1_R {
        TEIE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEIE2"]
    #[inline(always)]
    pub fn geie2(&self) -> GEIE2_R {
        GEIE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEIE2"]
    #[inline(always)]
    pub fn beie2(&self) -> BEIE2_R {
        BEIE2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTIE2"]
    #[inline(always)]
    pub fn htie2(&self) -> HTIE2_R {
        HTIE2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCIE2"]
    #[inline(always)]
    pub fn tcie2(&self) -> TCIE2_R {
        TCIE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEIE2"]
    #[inline(always)]
    pub fn teie2(&self) -> TEIE2_R {
        TEIE2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEIE3"]
    #[inline(always)]
    pub fn geie3(&self) -> GEIE3_R {
        GEIE3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEIE3"]
    #[inline(always)]
    pub fn beie3(&self) -> BEIE3_R {
        BEIE3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTIE3"]
    #[inline(always)]
    pub fn htie3(&self) -> HTIE3_R {
        HTIE3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCIE3"]
    #[inline(always)]
    pub fn tcie3(&self) -> TCIE3_R {
        TCIE3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIE3"]
    #[inline(always)]
    pub fn teie3(&self) -> TEIE3_R {
        TEIE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEIE4"]
    #[inline(always)]
    pub fn geie4(&self) -> GEIE4_R {
        GEIE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEIE4"]
    #[inline(always)]
    pub fn beie4(&self) -> BEIE4_R {
        BEIE4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTIE4"]
    #[inline(always)]
    pub fn htie4(&self) -> HTIE4_R {
        HTIE4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCIE4"]
    #[inline(always)]
    pub fn tcie4(&self) -> TCIE4_R {
        TCIE4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEIE4"]
    #[inline(always)]
    pub fn teie4(&self) -> TEIE4_R {
        TEIE4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEIE5"]
    #[inline(always)]
    pub fn geie5(&self) -> GEIE5_R {
        GEIE5_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEIE5"]
    #[inline(always)]
    pub fn beie5(&self) -> BEIE5_R {
        BEIE5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTIE5"]
    #[inline(always)]
    pub fn htie5(&self) -> HTIE5_R {
        HTIE5_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCIE5"]
    #[inline(always)]
    pub fn tcie5(&self) -> TCIE5_R {
        TCIE5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEIE5"]
    #[inline(always)]
    pub fn teie5(&self) -> TEIE5_R {
        TEIE5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEIE0"]
    #[inline(always)]
    pub fn geie0(&mut self) -> GEIE0_W {
        GEIE0_W { w: self }
    }
    #[doc = "Bit 1 - BEIE0"]
    #[inline(always)]
    pub fn beie0(&mut self) -> BEIE0_W {
        BEIE0_W { w: self }
    }
    #[doc = "Bit 2 - HTIE0"]
    #[inline(always)]
    pub fn htie0(&mut self) -> HTIE0_W {
        HTIE0_W { w: self }
    }
    #[doc = "Bit 3 - TCIE0"]
    #[inline(always)]
    pub fn tcie0(&mut self) -> TCIE0_W {
        TCIE0_W { w: self }
    }
    #[doc = "Bit 4 - TEIE0"]
    #[inline(always)]
    pub fn teie0(&mut self) -> TEIE0_W {
        TEIE0_W { w: self }
    }
    #[doc = "Bit 5 - GEIE1"]
    #[inline(always)]
    pub fn geie1(&mut self) -> GEIE1_W {
        GEIE1_W { w: self }
    }
    #[doc = "Bit 6 - BEIE1"]
    #[inline(always)]
    pub fn beie1(&mut self) -> BEIE1_W {
        BEIE1_W { w: self }
    }
    #[doc = "Bit 7 - HTIE1"]
    #[inline(always)]
    pub fn htie1(&mut self) -> HTIE1_W {
        HTIE1_W { w: self }
    }
    #[doc = "Bit 8 - TCIE1"]
    #[inline(always)]
    pub fn tcie1(&mut self) -> TCIE1_W {
        TCIE1_W { w: self }
    }
    #[doc = "Bit 9 - TEIE1"]
    #[inline(always)]
    pub fn teie1(&mut self) -> TEIE1_W {
        TEIE1_W { w: self }
    }
    #[doc = "Bit 10 - GEIE2"]
    #[inline(always)]
    pub fn geie2(&mut self) -> GEIE2_W {
        GEIE2_W { w: self }
    }
    #[doc = "Bit 11 - BEIE2"]
    #[inline(always)]
    pub fn beie2(&mut self) -> BEIE2_W {
        BEIE2_W { w: self }
    }
    #[doc = "Bit 12 - HTIE2"]
    #[inline(always)]
    pub fn htie2(&mut self) -> HTIE2_W {
        HTIE2_W { w: self }
    }
    #[doc = "Bit 13 - TCIE2"]
    #[inline(always)]
    pub fn tcie2(&mut self) -> TCIE2_W {
        TCIE2_W { w: self }
    }
    #[doc = "Bit 14 - TEIE2"]
    #[inline(always)]
    pub fn teie2(&mut self) -> TEIE2_W {
        TEIE2_W { w: self }
    }
    #[doc = "Bit 15 - GEIE3"]
    #[inline(always)]
    pub fn geie3(&mut self) -> GEIE3_W {
        GEIE3_W { w: self }
    }
    #[doc = "Bit 16 - BEIE3"]
    #[inline(always)]
    pub fn beie3(&mut self) -> BEIE3_W {
        BEIE3_W { w: self }
    }
    #[doc = "Bit 17 - HTIE3"]
    #[inline(always)]
    pub fn htie3(&mut self) -> HTIE3_W {
        HTIE3_W { w: self }
    }
    #[doc = "Bit 18 - TCIE3"]
    #[inline(always)]
    pub fn tcie3(&mut self) -> TCIE3_W {
        TCIE3_W { w: self }
    }
    #[doc = "Bit 19 - TEIE3"]
    #[inline(always)]
    pub fn teie3(&mut self) -> TEIE3_W {
        TEIE3_W { w: self }
    }
    #[doc = "Bit 20 - GEIE4"]
    #[inline(always)]
    pub fn geie4(&mut self) -> GEIE4_W {
        GEIE4_W { w: self }
    }
    #[doc = "Bit 21 - BEIE4"]
    #[inline(always)]
    pub fn beie4(&mut self) -> BEIE4_W {
        BEIE4_W { w: self }
    }
    #[doc = "Bit 22 - HTIE4"]
    #[inline(always)]
    pub fn htie4(&mut self) -> HTIE4_W {
        HTIE4_W { w: self }
    }
    #[doc = "Bit 23 - TCIE4"]
    #[inline(always)]
    pub fn tcie4(&mut self) -> TCIE4_W {
        TCIE4_W { w: self }
    }
    #[doc = "Bit 24 - TEIE4"]
    #[inline(always)]
    pub fn teie4(&mut self) -> TEIE4_W {
        TEIE4_W { w: self }
    }
    #[doc = "Bit 25 - GEIE5"]
    #[inline(always)]
    pub fn geie5(&mut self) -> GEIE5_W {
        GEIE5_W { w: self }
    }
    #[doc = "Bit 26 - BEIE5"]
    #[inline(always)]
    pub fn beie5(&mut self) -> BEIE5_W {
        BEIE5_W { w: self }
    }
    #[doc = "Bit 27 - HTIE5"]
    #[inline(always)]
    pub fn htie5(&mut self) -> HTIE5_W {
        HTIE5_W { w: self }
    }
    #[doc = "Bit 28 - TCIE5"]
    #[inline(always)]
    pub fn tcie5(&mut self) -> TCIE5_W {
        TCIE5_W { w: self }
    }
    #[doc = "Bit 29 - TEIE5"]
    #[inline(always)]
    pub fn teie5(&mut self) -> TEIE5_W {
        TEIE5_W { w: self }
    }
}
