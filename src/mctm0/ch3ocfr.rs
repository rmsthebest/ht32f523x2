#[doc = "Register `CH3OCFR` reader"]
pub struct R(crate::R<CH3OCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3OCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3OCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3OCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3OCFR` writer"]
pub struct W(crate::W<CH3OCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3OCFR_SPEC>;
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
impl From<crate::W<CH3OCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3OCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3OM` reader - CH3OM"]
pub type CH3OM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3OM` writer - CH3OM"]
pub type CH3OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3OCFR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CH3PRE` reader - CH3PRE"]
pub type CH3PRE_R = crate::BitReader<bool>;
#[doc = "Field `CH3PRE` writer - CH3PRE"]
pub type CH3PRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3OCFR_SPEC, bool, O>;
#[doc = "Field `CH3IMAE` reader - CH3IMAE"]
pub type CH3IMAE_R = crate::BitReader<bool>;
#[doc = "Field `CH3IMAE` writer - CH3IMAE"]
pub type CH3IMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3OCFR_SPEC, bool, O>;
#[doc = "Field `CH3OM3` reader - CH3OM3"]
pub type CH3OM3_R = crate::BitReader<bool>;
#[doc = "Field `CH3OM3` writer - CH3OM3"]
pub type CH3OM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3OCFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    pub fn ch3pre(&self) -> CH3PRE_R {
        CH3PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    pub fn ch3imae(&self) -> CH3IMAE_R {
        CH3IMAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    pub fn ch3om3(&self) -> CH3OM3_R {
        CH3OM3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH3OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<0> {
        CH3OM_W::new(self)
    }
    #[doc = "Bit 4 - CH3PRE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pre(&mut self) -> CH3PRE_W<4> {
        CH3PRE_W::new(self)
    }
    #[doc = "Bit 5 - CH3IMAE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3imae(&mut self) -> CH3IMAE_W<5> {
        CH3IMAE_W::new(self)
    }
    #[doc = "Bit 8 - CH3OM3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om3(&mut self) -> CH3OM3_W<8> {
        CH3OM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH3OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ocfr](index.html) module"]
pub struct CH3OCFR_SPEC;
impl crate::RegisterSpec for CH3OCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3ocfr::R](R) reader structure"]
impl crate::Readable for CH3OCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3ocfr::W](W) writer structure"]
impl crate::Writable for CH3OCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3OCFR to value 0"]
impl crate::Resettable for CH3OCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
