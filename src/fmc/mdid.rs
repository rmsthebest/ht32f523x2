#[doc = "Reader of register MDID"]
pub type R = crate::R<u32, super::MDID>;
#[doc = "Writer for register MDID"]
pub type W = crate::W<u32, super::MDID>;
#[doc = "Register MDID `reset()`'s with value 0"]
impl crate::ResetValue for super::MDID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHIPID`"]
pub type CHIPID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHIPID`"]
pub struct CHIPID_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MFID`"]
pub type MFID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFID`"]
pub struct MFID_W<'a> {
    w: &'a mut W,
}
impl<'a> MFID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    pub fn chipid(&mut self) -> CHIPID_W {
        CHIPID_W { w: self }
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&mut self) -> MFID_W {
        MFID_W { w: self }
    }
}
