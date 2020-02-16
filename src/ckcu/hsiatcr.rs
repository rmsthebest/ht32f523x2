#[doc = "Reader of register HSIATCR"]
pub type R = crate::R<u32, super::HSIATCR>;
#[doc = "Writer for register HSIATCR"]
pub type W = crate::W<u32, super::HSIATCR>;
#[doc = "Register HSIATCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSIATCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATCNT`"]
pub type ATCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ATCNT`"]
pub struct ATCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    pub fn atcnt(&self) -> ATCNT_R {
        ATCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    pub fn atcnt(&mut self) -> ATCNT_W {
        ATCNT_W { w: self }
    }
}
