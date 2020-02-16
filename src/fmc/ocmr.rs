#[doc = "Reader of register OCMR"]
pub type R = crate::R<u32, super::OCMR>;
#[doc = "Writer for register OCMR"]
pub type W = crate::W<u32, super::OCMR>;
#[doc = "Register OCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::OCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
