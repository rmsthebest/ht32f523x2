#[doc = "Reader of register CDR"]
pub type R = crate::R<u32, super::CDR>;
#[doc = "Writer for register CDR"]
pub type W = crate::W<u32, super::CDR>;
#[doc = "Register CDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Y_DIV`"]
pub type Y_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_DIV`"]
pub struct Y_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `X_DIV`"]
pub type X_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X_DIV`"]
pub struct X_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> X_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `N_DIV`"]
pub type N_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `N_DIV`"]
pub struct N_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> N_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    pub fn y_div(&self) -> Y_DIV_R {
        Y_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    pub fn x_div(&self) -> X_DIV_R {
        X_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    pub fn n_div(&self) -> N_DIV_R {
        N_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    pub fn y_div(&mut self) -> Y_DIV_W {
        Y_DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    pub fn x_div(&mut self) -> X_DIV_W {
        X_DIV_W { w: self }
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    pub fn n_div(&mut self) -> N_DIV_W {
        N_DIV_W { w: self }
    }
}
