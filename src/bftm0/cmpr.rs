#[doc = "Reader of register CMPR"]
pub type R = crate::R<u32, super::CMPR>;
#[doc = "Writer for register CMPR"]
pub type W = crate::W<u32, super::CMPR>;
#[doc = "Register CMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP`"]
pub type CMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMP`"]
pub struct CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W { w: self }
    }
}
