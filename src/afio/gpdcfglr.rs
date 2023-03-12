#[doc = "Register `GPDCFGLR` reader"]
pub struct R(crate::R<GPDCFGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDCFGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDCFGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDCFGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPDCFGLR` writer"]
pub struct W(crate::W<GPDCFGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDCFGLR_SPEC>;
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
impl From<crate::W<GPDCFGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDCFGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG0` reader - CFG0"]
pub type CFG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG0` writer - CFG0"]
pub type CFG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG1` reader - CFG1"]
pub type CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG1` writer - CFG1"]
pub type CFG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG2` reader - CFG2"]
pub type CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG2` writer - CFG2"]
pub type CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG3` reader - CFG3"]
pub type CFG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG3` writer - CFG3"]
pub type CFG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG4` reader - CFG4"]
pub type CFG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG4` writer - CFG4"]
pub type CFG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG5` reader - CFG5"]
pub type CFG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG5` writer - CFG5"]
pub type CFG5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG6` reader - CFG6"]
pub type CFG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG6` writer - CFG6"]
pub type CFG6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG7` reader - CFG7"]
pub type CFG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG7` writer - CFG7"]
pub type CFG7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDCFGLR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    #[must_use]
    pub fn cfg0(&mut self) -> CFG0_W<0> {
        CFG0_W::new(self)
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<4> {
        CFG1_W::new(self)
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<8> {
        CFG2_W::new(self)
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    #[must_use]
    pub fn cfg3(&mut self) -> CFG3_W<12> {
        CFG3_W::new(self)
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    #[must_use]
    pub fn cfg4(&mut self) -> CFG4_W<16> {
        CFG4_W::new(self)
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    #[must_use]
    pub fn cfg5(&mut self) -> CFG5_W<20> {
        CFG5_W::new(self)
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    #[must_use]
    pub fn cfg6(&mut self) -> CFG6_W<24> {
        CFG6_W::new(self)
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg7(&mut self) -> CFG7_W<28> {
        CFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdcfglr](index.html) module"]
pub struct GPDCFGLR_SPEC;
impl crate::RegisterSpec for GPDCFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdcfglr::R](R) reader structure"]
impl crate::Readable for GPDCFGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdcfglr::W](W) writer structure"]
impl crate::Writable for GPDCFGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPDCFGLR to value 0"]
impl crate::Resettable for GPDCFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
