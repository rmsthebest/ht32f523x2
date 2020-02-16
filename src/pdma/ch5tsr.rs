#[doc = "Reader of register CH5TSR"]
pub type R = crate::R<u32, super::CH5TSR>;
#[doc = "Writer for register CH5TSR"]
pub type W = crate::W<u32, super::CH5TSR>;
#[doc = "Register CH5TSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH5TSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLKLEN`"]
pub type BLKLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLKLEN`"]
pub struct BLKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BLKCNT`"]
pub type BLKCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLKCNT`"]
pub struct BLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    pub fn blklen(&mut self) -> BLKLEN_W {
        BLKLEN_W { w: self }
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    pub fn blkcnt(&mut self) -> BLKCNT_W {
        BLKCNT_W { w: self }
    }
}
