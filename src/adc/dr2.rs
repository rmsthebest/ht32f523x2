#[doc = "Register `DR2` reader"]
pub struct R(crate::R<DR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR2` writer"]
pub struct W(crate::W<DR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR2_SPEC>;
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
impl From<crate::W<DR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD2` reader - ADD2"]
pub type ADD2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD2` writer - ADD2"]
pub type ADD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR2_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADVLD2` reader - ADVLD2"]
pub type ADVLD2_R = crate::BitReader<bool>;
#[doc = "Field `ADVLD2` writer - ADVLD2"]
pub type ADVLD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    pub fn advld2(&self) -> ADVLD2_R {
        ADVLD2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD2"]
    #[inline(always)]
    #[must_use]
    pub fn add2(&mut self) -> ADD2_W<0> {
        ADD2_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD2"]
    #[inline(always)]
    #[must_use]
    pub fn advld2(&mut self) -> ADVLD2_W<31> {
        ADVLD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr2](index.html) module"]
pub struct DR2_SPEC;
impl crate::RegisterSpec for DR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr2::R](R) reader structure"]
impl crate::Readable for DR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr2::W](W) writer structure"]
impl crate::Writable for DR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR2 to value 0"]
impl crate::Resettable for DR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
