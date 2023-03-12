#[doc = "Register `DR1` reader"]
pub struct R(crate::R<DR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR1` writer"]
pub struct W(crate::W<DR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR1_SPEC>;
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
impl From<crate::W<DR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD1` reader - ADD1"]
pub type ADD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD1` writer - ADD1"]
pub type ADD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADVLD1` reader - ADVLD1"]
pub type ADVLD1_R = crate::BitReader<bool>;
#[doc = "Field `ADVLD1` writer - ADVLD1"]
pub type ADVLD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    pub fn advld1(&self) -> ADVLD1_R {
        ADVLD1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD1"]
    #[inline(always)]
    #[must_use]
    pub fn add1(&mut self) -> ADD1_W<0> {
        ADD1_W::new(self)
    }
    #[doc = "Bit 31 - ADVLD1"]
    #[inline(always)]
    #[must_use]
    pub fn advld1(&mut self) -> ADVLD1_W<31> {
        ADVLD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr1](index.html) module"]
pub struct DR1_SPEC;
impl crate::RegisterSpec for DR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr1::R](R) reader structure"]
impl crate::Readable for DR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr1::W](W) writer structure"]
impl crate::Writable for DR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for DR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
