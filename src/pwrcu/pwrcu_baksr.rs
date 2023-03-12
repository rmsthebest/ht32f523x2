#[doc = "Register `PWRCU_BAKSR` reader"]
pub struct R(crate::R<PWRCU_BAKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_BAKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_BAKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_BAKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_BAKSR` writer"]
pub struct W(crate::W<PWRCU_BAKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_BAKSR_SPEC>;
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
impl From<crate::W<PWRCU_BAKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_BAKSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAKPORF` reader - BAKPORF"]
pub type BAKPORF_R = crate::BitReader<bool>;
#[doc = "Field `BAKPORF` writer - BAKPORF"]
pub type BAKPORF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_BAKSR_SPEC, bool, O>;
#[doc = "Field `PDF` reader - PDF"]
pub type PDF_R = crate::BitReader<bool>;
#[doc = "Field `PDF` writer - PDF"]
pub type PDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_BAKSR_SPEC, bool, O>;
#[doc = "Field `WUPF` reader - WUPF"]
pub type WUPF_R = crate::BitReader<bool>;
#[doc = "Field `WUPF` writer - WUPF"]
pub type WUPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCU_BAKSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    pub fn bakporf(&self) -> BAKPORF_R {
        BAKPORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    pub fn pdf(&self) -> PDF_R {
        PDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    pub fn wupf(&self) -> WUPF_R {
        WUPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    #[must_use]
    pub fn bakporf(&mut self) -> BAKPORF_W<0> {
        BAKPORF_W::new(self)
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    #[must_use]
    pub fn pdf(&mut self) -> PDF_W<1> {
        PDF_W::new(self)
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    #[must_use]
    pub fn wupf(&mut self) -> WUPF_W<8> {
        WUPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_BAKSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_baksr](index.html) module"]
pub struct PWRCU_BAKSR_SPEC;
impl crate::RegisterSpec for PWRCU_BAKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_baksr::R](R) reader structure"]
impl crate::Readable for PWRCU_BAKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_baksr::W](W) writer structure"]
impl crate::Writable for PWRCU_BAKSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_BAKSR to value 0"]
impl crate::Resettable for PWRCU_BAKSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
