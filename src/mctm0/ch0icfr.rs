#[doc = "Register `CH0ICFR` reader"]
pub struct R(crate::R<CH0ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0ICFR` writer"]
pub struct W(crate::W<CH0ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH0ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI0F` reader - TI0F"]
pub type TI0F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI0F` writer - TI0F"]
pub type TI0F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0ICFR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH0CCS` reader - CH0CCS"]
pub type CH0CCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0CCS` writer - CH0CCS"]
pub type CH0CCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0ICFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH0PSC` reader - CH0PSC"]
pub type CH0PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0PSC` writer - CH0PSC"]
pub type CH0PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0ICFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TI0SRC` reader - TI0SRC"]
pub type TI0SRC_R = crate::BitReader<bool>;
#[doc = "Field `TI0SRC` writer - TI0SRC"]
pub type TI0SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0ICFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    pub fn ti0f(&self) -> TI0F_R {
        TI0F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    pub fn ch0ccs(&self) -> CH0CCS_R {
        CH0CCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    pub fn ch0psc(&self) -> CH0PSC_R {
        CH0PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    pub fn ti0src(&self) -> TI0SRC_R {
        TI0SRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI0F"]
    #[inline(always)]
    #[must_use]
    pub fn ti0f(&mut self) -> TI0F_W<0> {
        TI0F_W::new(self)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccs(&mut self) -> CH0CCS_W<16> {
        CH0CCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch0psc(&mut self) -> CH0PSC_W<18> {
        CH0PSC_W::new(self)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ti0src(&mut self) -> TI0SRC_W<31> {
        TI0SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH0ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0icfr](index.html) module"]
pub struct CH0ICFR_SPEC;
impl crate::RegisterSpec for CH0ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0icfr::R](R) reader structure"]
impl crate::Readable for CH0ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0icfr::W](W) writer structure"]
impl crate::Writable for CH0ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0ICFR to value 0"]
impl crate::Resettable for CH0ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
