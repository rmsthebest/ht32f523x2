#[doc = "Reader of register WTR"]
pub type R = crate::R<u32, super::WTR>;
#[doc = "Writer for register WTR"]
pub type W = crate::W<u32, super::WTR>;
#[doc = "Register WTR `reset()`'s with value 0"]
impl crate::ResetValue for super::WTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WT`"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
}
