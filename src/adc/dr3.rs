#[doc = "Register `DR3` reader"]
pub struct R(crate::R<DR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR3` writer"]
pub struct W(crate::W<DR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR3_SPEC>;
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
impl From<crate::W<DR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD3` reader - ADD3"]
pub type ADD3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD3` writer - ADD3"]
pub type ADD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR3_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADVLD3` reader - ADVLD3"]
pub type ADVLD3_R = crate::BitReader<bool>;
#[doc = "Field `ADVLD3` writer - ADVLD3"]
pub type ADVLD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    pub fn add3(&self) -> ADD3_R {
        ADD3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    pub fn advld3(&self) -> ADVLD3_R {
        ADVLD3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD3"]
    #[inline(always)]
    #[must_use]
    pub fn add3(&mut self) -> ADD3_W<0> {
        ADD3_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD3"]
    #[inline(always)]
    #[must_use]
    pub fn advld3(&mut self) -> ADVLD3_W<31> {
        ADVLD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr3](index.html) module"]
pub struct DR3_SPEC;
impl crate::RegisterSpec for DR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr3::R](R) reader structure"]
impl crate::Readable for DR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr3::W](W) writer structure"]
impl crate::Writable for DR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for DR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
