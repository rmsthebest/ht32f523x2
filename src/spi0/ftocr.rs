#[doc = "Reader of register FTOCR"]
pub type R = crate::R<u32, super::FTOCR>;
#[doc = "Writer for register FTOCR"]
pub type W = crate::W<u32, super::FTOCR>;
#[doc = "Register FTOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FTOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOC`"]
pub type TOC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOC`"]
pub struct TOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
}
