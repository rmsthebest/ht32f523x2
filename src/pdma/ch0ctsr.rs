#[doc = "Reader of register CH0CTSR"]
pub type R = crate::R<u32, super::CH0CTSR>;
#[doc = "Writer for register CH0CTSR"]
pub type W = crate::W<u32, super::CH0CTSR>;
#[doc = "Register CH0CTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0CTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBLKCNT`"]
pub type CBLKCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CBLKCNT`"]
pub struct CBLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBLKCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&self) -> CBLKCNT_R {
        CBLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&mut self) -> CBLKCNT_W {
        CBLKCNT_W { w: self }
    }
}
