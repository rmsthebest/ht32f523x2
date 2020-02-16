#[doc = "Reader of register VMCR"]
pub type R = crate::R<u32, super::VMCR>;
#[doc = "Writer for register VMCR"]
pub type W = crate::W<u32, super::VMCR>;
#[doc = "Register VMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::VMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VMCB`"]
pub type VMCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMCB`"]
pub struct VMCB_W<'a> {
    w: &'a mut W,
}
impl<'a> VMCB_W<'a> {
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
impl R {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&self) -> VMCB_R {
        VMCB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&mut self) -> VMCB_W {
        VMCB_W { w: self }
    }
}
