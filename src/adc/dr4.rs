#[doc = "Register `DR4` reader"]
pub struct R(crate::R<DR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR4` writer"]
pub struct W(crate::W<DR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR4_SPEC>;
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
impl From<crate::W<DR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD4` reader - ADD4"]
pub type ADD4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD4` writer - ADD4"]
pub type ADD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR4_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADVLD4` reader - ADVLD4"]
pub type ADVLD4_R = crate::BitReader<bool>;
#[doc = "Field `ADVLD4` writer - ADVLD4"]
pub type ADVLD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    pub fn add4(&self) -> ADD4_R {
        ADD4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    pub fn advld4(&self) -> ADVLD4_R {
        ADVLD4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD4"]
    #[inline(always)]
    #[must_use]
    pub fn add4(&mut self) -> ADD4_W<0> {
        ADD4_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD4"]
    #[inline(always)]
    #[must_use]
    pub fn advld4(&mut self) -> ADVLD4_W<31> {
        ADVLD4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr4](index.html) module"]
pub struct DR4_SPEC;
impl crate::RegisterSpec for DR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr4::R](R) reader structure"]
impl crate::Readable for DR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr4::W](W) writer structure"]
impl crate::Writable for DR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR4 to value 0"]
impl crate::Resettable for DR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
