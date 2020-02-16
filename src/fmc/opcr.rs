#[doc = "Reader of register OPCR"]
pub type R = crate::R<u32, super::OPCR>;
#[doc = "Writer for register OPCR"]
pub type W = crate::W<u32, super::OPCR>;
#[doc = "Register OPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPM`"]
pub type OPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPM`"]
pub struct OPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W { w: self }
    }
}
