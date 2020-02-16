#[doc = "Reader of register CPR"]
pub type R = crate::R<u32, super::CPR>;
#[doc = "Writer for register CPR"]
pub type W = crate::W<u32, super::CPR>;
#[doc = "Register CPR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CP`"]
pub type CP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CP`"]
pub struct CP_W<'a> {
    w: &'a mut W,
}
impl<'a> CP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CP"]
    #[inline(always)]
    pub fn cp(&mut self) -> CP_W {
        CP_W { w: self }
    }
}
