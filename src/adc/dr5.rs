#[doc = "Register `DR5` reader"]
pub struct R(crate::R<DR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR5` writer"]
pub struct W(crate::W<DR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR5_SPEC>;
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
impl From<crate::W<DR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD5` reader - ADD5"]
pub type ADD5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD5` writer - ADD5"]
pub type ADD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR5_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADVLD5` reader - ADVLD5"]
pub type ADVLD5_R = crate::BitReader<bool>;
#[doc = "Field `ADVLD5` writer - ADVLD5"]
pub type ADVLD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    pub fn add5(&self) -> ADD5_R {
        ADD5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    pub fn advld5(&self) -> ADVLD5_R {
        ADVLD5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD5"]
    #[inline(always)]
    #[must_use]
    pub fn add5(&mut self) -> ADD5_W<0> {
        ADD5_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD5"]
    #[inline(always)]
    #[must_use]
    pub fn advld5(&mut self) -> ADVLD5_W<31> {
        ADVLD5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr5](index.html) module"]
pub struct DR5_SPEC;
impl crate::RegisterSpec for DR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr5::R](R) reader structure"]
impl crate::Readable for DR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr5::W](W) writer structure"]
impl crate::Writable for DR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR5 to value 0"]
impl crate::Resettable for DR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
