#[doc = "Reader of register TOUT"]
pub type R = crate::R<u32, super::TOUT>;
#[doc = "Writer for register TOUT"]
pub type W = crate::W<u32, super::TOUT>;
#[doc = "Register TOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::TOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOUT`"]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
