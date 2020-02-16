#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEISTA0`"]
pub type GEISTA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA0`"]
pub struct GEISTA0_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA0_W<'a> {
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
#[doc = "Reader of field `BEISTA0`"]
pub type BEISTA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA0`"]
pub struct BEISTA0_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA0_W<'a> {
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
#[doc = "Reader of field `HTISTA0`"]
pub type HTISTA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA0`"]
pub struct HTISTA0_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA0_W<'a> {
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
#[doc = "Reader of field `TCISTA0`"]
pub type TCISTA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA0`"]
pub struct TCISTA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA0_W<'a> {
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
#[doc = "Reader of field `TEISTA0`"]
pub type TEISTA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA0`"]
pub struct TEISTA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA0_W<'a> {
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
#[doc = "Reader of field `GEISTA1`"]
pub type GEISTA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA1`"]
pub struct GEISTA1_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA1_W<'a> {
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
#[doc = "Reader of field `BEISTA1`"]
pub type BEISTA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA1`"]
pub struct BEISTA1_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA1_W<'a> {
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
#[doc = "Reader of field `HTISTA1`"]
pub type HTISTA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA1`"]
pub struct HTISTA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA1_W<'a> {
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
#[doc = "Reader of field `TCISTA1`"]
pub type TCISTA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA1`"]
pub struct TCISTA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA1_W<'a> {
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
#[doc = "Reader of field `TEISTA1`"]
pub type TEISTA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA1`"]
pub struct TEISTA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA1_W<'a> {
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
#[doc = "Reader of field `GEISTA2`"]
pub type GEISTA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA2`"]
pub struct GEISTA2_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA2_W<'a> {
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
#[doc = "Reader of field `BEISTA2`"]
pub type BEISTA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA2`"]
pub struct BEISTA2_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA2_W<'a> {
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
#[doc = "Reader of field `HTISTA2`"]
pub type HTISTA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA2`"]
pub struct HTISTA2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA2_W<'a> {
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
#[doc = "Reader of field `TCISTA2`"]
pub type TCISTA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA2`"]
pub struct TCISTA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA2_W<'a> {
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
#[doc = "Reader of field `TEISTA2`"]
pub type TEISTA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA2`"]
pub struct TEISTA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA2_W<'a> {
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
#[doc = "Reader of field `GEISTA3`"]
pub type GEISTA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA3`"]
pub struct GEISTA3_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA3_W<'a> {
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
#[doc = "Reader of field `BEISTA3`"]
pub type BEISTA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA3`"]
pub struct BEISTA3_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA3_W<'a> {
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
#[doc = "Reader of field `HTISTA3`"]
pub type HTISTA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA3`"]
pub struct HTISTA3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA3_W<'a> {
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
#[doc = "Reader of field `TCISTA3`"]
pub type TCISTA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA3`"]
pub struct TCISTA3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA3_W<'a> {
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
#[doc = "Reader of field `TEISTA3`"]
pub type TEISTA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA3`"]
pub struct TEISTA3_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA3_W<'a> {
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
#[doc = "Reader of field `GEISTA4`"]
pub type GEISTA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA4`"]
pub struct GEISTA4_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA4_W<'a> {
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
#[doc = "Reader of field `BEISTA4`"]
pub type BEISTA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA4`"]
pub struct BEISTA4_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA4_W<'a> {
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
#[doc = "Reader of field `HTISTA4`"]
pub type HTISTA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA4`"]
pub struct HTISTA4_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA4_W<'a> {
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
#[doc = "Reader of field `TCISTA4`"]
pub type TCISTA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA4`"]
pub struct TCISTA4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA4_W<'a> {
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
#[doc = "Reader of field `TEISTA4`"]
pub type TEISTA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA4`"]
pub struct TEISTA4_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA4_W<'a> {
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
#[doc = "Reader of field `GEISTA5`"]
pub type GEISTA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA5`"]
pub struct GEISTA5_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA5_W<'a> {
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
#[doc = "Reader of field `BEISTA5`"]
pub type BEISTA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA5`"]
pub struct BEISTA5_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA5_W<'a> {
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
#[doc = "Reader of field `HTISTA5`"]
pub type HTISTA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA5`"]
pub struct HTISTA5_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA5_W<'a> {
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
#[doc = "Reader of field `TCISTA5`"]
pub type TCISTA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA5`"]
pub struct TCISTA5_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA5_W<'a> {
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
#[doc = "Reader of field `TEISTA5`"]
pub type TEISTA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA5`"]
pub struct TEISTA5_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA5_W<'a> {
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
    #[doc = "Bit 0 - GEISTA0"]
    #[inline(always)]
    pub fn geista0(&self) -> GEISTA0_R {
        GEISTA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEISTA0"]
    #[inline(always)]
    pub fn beista0(&self) -> BEISTA0_R {
        BEISTA0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTISTA0"]
    #[inline(always)]
    pub fn htista0(&self) -> HTISTA0_R {
        HTISTA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCISTA0"]
    #[inline(always)]
    pub fn tcista0(&self) -> TCISTA0_R {
        TCISTA0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEISTA0"]
    #[inline(always)]
    pub fn teista0(&self) -> TEISTA0_R {
        TEISTA0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEISTA1"]
    #[inline(always)]
    pub fn geista1(&self) -> GEISTA1_R {
        GEISTA1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEISTA1"]
    #[inline(always)]
    pub fn beista1(&self) -> BEISTA1_R {
        BEISTA1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTISTA1"]
    #[inline(always)]
    pub fn htista1(&self) -> HTISTA1_R {
        HTISTA1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCISTA1"]
    #[inline(always)]
    pub fn tcista1(&self) -> TCISTA1_R {
        TCISTA1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEISTA1"]
    #[inline(always)]
    pub fn teista1(&self) -> TEISTA1_R {
        TEISTA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEISTA2"]
    #[inline(always)]
    pub fn geista2(&self) -> GEISTA2_R {
        GEISTA2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEISTA2"]
    #[inline(always)]
    pub fn beista2(&self) -> BEISTA2_R {
        BEISTA2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTISTA2"]
    #[inline(always)]
    pub fn htista2(&self) -> HTISTA2_R {
        HTISTA2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCISTA2"]
    #[inline(always)]
    pub fn tcista2(&self) -> TCISTA2_R {
        TCISTA2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEISTA2"]
    #[inline(always)]
    pub fn teista2(&self) -> TEISTA2_R {
        TEISTA2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEISTA3"]
    #[inline(always)]
    pub fn geista3(&self) -> GEISTA3_R {
        GEISTA3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEISTA3"]
    #[inline(always)]
    pub fn beista3(&self) -> BEISTA3_R {
        BEISTA3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTISTA3"]
    #[inline(always)]
    pub fn htista3(&self) -> HTISTA3_R {
        HTISTA3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCISTA3"]
    #[inline(always)]
    pub fn tcista3(&self) -> TCISTA3_R {
        TCISTA3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEISTA3"]
    #[inline(always)]
    pub fn teista3(&self) -> TEISTA3_R {
        TEISTA3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEISTA4"]
    #[inline(always)]
    pub fn geista4(&self) -> GEISTA4_R {
        GEISTA4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEISTA4"]
    #[inline(always)]
    pub fn beista4(&self) -> BEISTA4_R {
        BEISTA4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTISTA4"]
    #[inline(always)]
    pub fn htista4(&self) -> HTISTA4_R {
        HTISTA4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCISTA4"]
    #[inline(always)]
    pub fn tcista4(&self) -> TCISTA4_R {
        TCISTA4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEISTA4"]
    #[inline(always)]
    pub fn teista4(&self) -> TEISTA4_R {
        TEISTA4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEISTA5"]
    #[inline(always)]
    pub fn geista5(&self) -> GEISTA5_R {
        GEISTA5_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEISTA5"]
    #[inline(always)]
    pub fn beista5(&self) -> BEISTA5_R {
        BEISTA5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTISTA5"]
    #[inline(always)]
    pub fn htista5(&self) -> HTISTA5_R {
        HTISTA5_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCISTA5"]
    #[inline(always)]
    pub fn tcista5(&self) -> TCISTA5_R {
        TCISTA5_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEISTA5"]
    #[inline(always)]
    pub fn teista5(&self) -> TEISTA5_R {
        TEISTA5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEISTA0"]
    #[inline(always)]
    pub fn geista0(&mut self) -> GEISTA0_W {
        GEISTA0_W { w: self }
    }
    #[doc = "Bit 1 - BEISTA0"]
    #[inline(always)]
    pub fn beista0(&mut self) -> BEISTA0_W {
        BEISTA0_W { w: self }
    }
    #[doc = "Bit 2 - HTISTA0"]
    #[inline(always)]
    pub fn htista0(&mut self) -> HTISTA0_W {
        HTISTA0_W { w: self }
    }
    #[doc = "Bit 3 - TCISTA0"]
    #[inline(always)]
    pub fn tcista0(&mut self) -> TCISTA0_W {
        TCISTA0_W { w: self }
    }
    #[doc = "Bit 4 - TEISTA0"]
    #[inline(always)]
    pub fn teista0(&mut self) -> TEISTA0_W {
        TEISTA0_W { w: self }
    }
    #[doc = "Bit 5 - GEISTA1"]
    #[inline(always)]
    pub fn geista1(&mut self) -> GEISTA1_W {
        GEISTA1_W { w: self }
    }
    #[doc = "Bit 6 - BEISTA1"]
    #[inline(always)]
    pub fn beista1(&mut self) -> BEISTA1_W {
        BEISTA1_W { w: self }
    }
    #[doc = "Bit 7 - HTISTA1"]
    #[inline(always)]
    pub fn htista1(&mut self) -> HTISTA1_W {
        HTISTA1_W { w: self }
    }
    #[doc = "Bit 8 - TCISTA1"]
    #[inline(always)]
    pub fn tcista1(&mut self) -> TCISTA1_W {
        TCISTA1_W { w: self }
    }
    #[doc = "Bit 9 - TEISTA1"]
    #[inline(always)]
    pub fn teista1(&mut self) -> TEISTA1_W {
        TEISTA1_W { w: self }
    }
    #[doc = "Bit 10 - GEISTA2"]
    #[inline(always)]
    pub fn geista2(&mut self) -> GEISTA2_W {
        GEISTA2_W { w: self }
    }
    #[doc = "Bit 11 - BEISTA2"]
    #[inline(always)]
    pub fn beista2(&mut self) -> BEISTA2_W {
        BEISTA2_W { w: self }
    }
    #[doc = "Bit 12 - HTISTA2"]
    #[inline(always)]
    pub fn htista2(&mut self) -> HTISTA2_W {
        HTISTA2_W { w: self }
    }
    #[doc = "Bit 13 - TCISTA2"]
    #[inline(always)]
    pub fn tcista2(&mut self) -> TCISTA2_W {
        TCISTA2_W { w: self }
    }
    #[doc = "Bit 14 - TEISTA2"]
    #[inline(always)]
    pub fn teista2(&mut self) -> TEISTA2_W {
        TEISTA2_W { w: self }
    }
    #[doc = "Bit 15 - GEISTA3"]
    #[inline(always)]
    pub fn geista3(&mut self) -> GEISTA3_W {
        GEISTA3_W { w: self }
    }
    #[doc = "Bit 16 - BEISTA3"]
    #[inline(always)]
    pub fn beista3(&mut self) -> BEISTA3_W {
        BEISTA3_W { w: self }
    }
    #[doc = "Bit 17 - HTISTA3"]
    #[inline(always)]
    pub fn htista3(&mut self) -> HTISTA3_W {
        HTISTA3_W { w: self }
    }
    #[doc = "Bit 18 - TCISTA3"]
    #[inline(always)]
    pub fn tcista3(&mut self) -> TCISTA3_W {
        TCISTA3_W { w: self }
    }
    #[doc = "Bit 19 - TEISTA3"]
    #[inline(always)]
    pub fn teista3(&mut self) -> TEISTA3_W {
        TEISTA3_W { w: self }
    }
    #[doc = "Bit 20 - GEISTA4"]
    #[inline(always)]
    pub fn geista4(&mut self) -> GEISTA4_W {
        GEISTA4_W { w: self }
    }
    #[doc = "Bit 21 - BEISTA4"]
    #[inline(always)]
    pub fn beista4(&mut self) -> BEISTA4_W {
        BEISTA4_W { w: self }
    }
    #[doc = "Bit 22 - HTISTA4"]
    #[inline(always)]
    pub fn htista4(&mut self) -> HTISTA4_W {
        HTISTA4_W { w: self }
    }
    #[doc = "Bit 23 - TCISTA4"]
    #[inline(always)]
    pub fn tcista4(&mut self) -> TCISTA4_W {
        TCISTA4_W { w: self }
    }
    #[doc = "Bit 24 - TEISTA4"]
    #[inline(always)]
    pub fn teista4(&mut self) -> TEISTA4_W {
        TEISTA4_W { w: self }
    }
    #[doc = "Bit 25 - GEISTA5"]
    #[inline(always)]
    pub fn geista5(&mut self) -> GEISTA5_W {
        GEISTA5_W { w: self }
    }
    #[doc = "Bit 26 - BEISTA5"]
    #[inline(always)]
    pub fn beista5(&mut self) -> BEISTA5_W {
        BEISTA5_W { w: self }
    }
    #[doc = "Bit 27 - HTISTA5"]
    #[inline(always)]
    pub fn htista5(&mut self) -> HTISTA5_W {
        HTISTA5_W { w: self }
    }
    #[doc = "Bit 28 - TCISTA5"]
    #[inline(always)]
    pub fn tcista5(&mut self) -> TCISTA5_W {
        TCISTA5_W { w: self }
    }
    #[doc = "Bit 29 - TEISTA5"]
    #[inline(always)]
    pub fn teista5(&mut self) -> TEISTA5_W {
        TEISTA5_W { w: self }
    }
}
