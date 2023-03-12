#[doc = "Register `GRSR` reader"]
pub struct R(crate::R<GRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRSR` writer"]
pub struct W(crate::W<GRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSR_SPEC>;
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
impl From<crate::W<GRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVICRSTF` reader - NVICRSTF"]
pub type NVICRSTF_R = crate::BitReader<bool>;
#[doc = "Field `NVICRSTF` writer - NVICRSTF"]
pub type NVICRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSR_SPEC, bool, O>;
#[doc = "Field `EXTRSTF` reader - EXTRSTF"]
pub type EXTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `EXTRSTF` writer - EXTRSTF"]
pub type EXTRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSR_SPEC, bool, O>;
#[doc = "Field `WDTRSTF` reader - WDTRSTF"]
pub type WDTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WDTRSTF` writer - WDTRSTF"]
pub type WDTRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSR_SPEC, bool, O>;
#[doc = "Field `PORSTF` reader - PORSTF"]
pub type PORSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORSTF` writer - PORSTF"]
pub type PORSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    pub fn nvicrstf(&self) -> NVICRSTF_R {
        NVICRSTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    pub fn extrstf(&self) -> EXTRSTF_R {
        EXTRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WDTRSTF_R {
        WDTRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    pub fn porstf(&self) -> PORSTF_R {
        PORSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVICRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn nvicrstf(&mut self) -> NVICRSTF_W<0> {
        NVICRSTF_W::new(self)
    }
    #[doc = "Bit 1 - EXTRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn extrstf(&mut self) -> EXTRSTF_W<1> {
        EXTRSTF_W::new(self)
    }
    #[doc = "Bit 2 - WDTRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrstf(&mut self) -> WDTRSTF_W<2> {
        WDTRSTF_W::new(self)
    }
    #[doc = "Bit 3 - PORSTF"]
    #[inline(always)]
    #[must_use]
    pub fn porstf(&mut self) -> PORSTF_W<3> {
        PORSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grsr](index.html) module"]
pub struct GRSR_SPEC;
impl crate::RegisterSpec for GRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grsr::R](R) reader structure"]
impl crate::Readable for GRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grsr::W](W) writer structure"]
impl crate::Writable for GRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRSR to value 0"]
impl crate::Resettable for GRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
