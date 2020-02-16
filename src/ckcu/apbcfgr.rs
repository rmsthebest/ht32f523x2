#[doc = "Reader of register APBCFGR"]
pub type R = crate::R<u32, super::APBCFGR>;
#[doc = "Writer for register APBCFGR"]
pub type W = crate::W<u32, super::APBCFGR>;
#[doc = "Register APBCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::APBCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCDIV`"]
pub type ADCDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCDIV`"]
pub struct ADCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    pub fn adcdiv(&self) -> ADCDIV_R {
        ADCDIV_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ADCDIV"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> ADCDIV_W {
        ADCDIV_W { w: self }
    }
}
