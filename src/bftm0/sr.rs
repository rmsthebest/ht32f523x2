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
#[doc = "Reader of field `MIF`"]
pub type MIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIF`"]
pub struct MIF_W<'a> {
    w: &'a mut W,
}
impl<'a> MIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    pub fn mif(&self) -> MIF_R {
        MIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    pub fn mif(&mut self) -> MIF_W {
        MIF_W { w: self }
    }
}
