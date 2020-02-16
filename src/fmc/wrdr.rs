#[doc = "Reader of register WRDR"]
pub type R = crate::R<u32, super::WRDR>;
#[doc = "Writer for register WRDR"]
pub type W = crate::W<u32, super::WRDR>;
#[doc = "Register WRDR `reset()`'s with value 0"]
impl crate::ResetValue for super::WRDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRDB`"]
pub type WRDB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRDB`"]
pub struct WRDB_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    pub fn wrdb(&self) -> WRDB_R {
        WRDB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    pub fn wrdb(&mut self) -> WRDB_W {
        WRDB_W { w: self }
    }
}
