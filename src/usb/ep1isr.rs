#[doc = "Register `EP1ISR` reader"]
pub struct R(crate::R<EP1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP1ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP1ISR` writer"]
pub struct W(crate::W<EP1ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP1ISR_SPEC>;
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
impl From<crate::W<EP1ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP1ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTRXIF` reader - OTRXIF"]
pub type OTRXIF_R = crate::BitReader<bool>;
#[doc = "Field `OTRXIF` writer - OTRXIF"]
pub type OTRXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `ODRXIF` reader - ODRXIF"]
pub type ODRXIF_R = crate::BitReader<bool>;
#[doc = "Field `ODRXIF` writer - ODRXIF"]
pub type ODRXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `ODOVIF` reader - ODOVIF"]
pub type ODOVIF_R = crate::BitReader<bool>;
#[doc = "Field `ODOVIF` writer - ODOVIF"]
pub type ODOVIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `ITRXIF` reader - ITRXIF"]
pub type ITRXIF_R = crate::BitReader<bool>;
#[doc = "Field `ITRXIF` writer - ITRXIF"]
pub type ITRXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `IDTXIF` reader - IDTXIF"]
pub type IDTXIF_R = crate::BitReader<bool>;
#[doc = "Field `IDTXIF` writer - IDTXIF"]
pub type IDTXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `NAKIF` reader - NAKIF"]
pub type NAKIF_R = crate::BitReader<bool>;
#[doc = "Field `NAKIF` writer - NAKIF"]
pub type NAKIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `STLIF` reader - STLIF"]
pub type STLIF_R = crate::BitReader<bool>;
#[doc = "Field `STLIF` writer - STLIF"]
pub type STLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
#[doc = "Field `UERIF` reader - UERIF"]
pub type UERIF_R = crate::BitReader<bool>;
#[doc = "Field `UERIF` writer - UERIF"]
pub type UERIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    pub fn otrxif(&self) -> OTRXIF_R {
        OTRXIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    pub fn odrxif(&self) -> ODRXIF_R {
        ODRXIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    pub fn odovif(&self) -> ODOVIF_R {
        ODOVIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    pub fn itrxif(&self) -> ITRXIF_R {
        ITRXIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    pub fn idtxif(&self) -> IDTXIF_R {
        IDTXIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    pub fn nakif(&self) -> NAKIF_R {
        NAKIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    pub fn stlif(&self) -> STLIF_R {
        STLIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    pub fn uerif(&self) -> UERIF_R {
        UERIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn otrxif(&mut self) -> OTRXIF_W<0> {
        OTRXIF_W::new(self)
    }
    #[doc = "Bit 1 - ODRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn odrxif(&mut self) -> ODRXIF_W<1> {
        ODRXIF_W::new(self)
    }
    #[doc = "Bit 2 - ODOVIF"]
    #[inline(always)]
    #[must_use]
    pub fn odovif(&mut self) -> ODOVIF_W<2> {
        ODOVIF_W::new(self)
    }
    #[doc = "Bit 3 - ITRXIF"]
    #[inline(always)]
    #[must_use]
    pub fn itrxif(&mut self) -> ITRXIF_W<3> {
        ITRXIF_W::new(self)
    }
    #[doc = "Bit 4 - IDTXIF"]
    #[inline(always)]
    #[must_use]
    pub fn idtxif(&mut self) -> IDTXIF_W<4> {
        IDTXIF_W::new(self)
    }
    #[doc = "Bit 5 - NAKIF"]
    #[inline(always)]
    #[must_use]
    pub fn nakif(&mut self) -> NAKIF_W<5> {
        NAKIF_W::new(self)
    }
    #[doc = "Bit 6 - STLIF"]
    #[inline(always)]
    #[must_use]
    pub fn stlif(&mut self) -> STLIF_W<6> {
        STLIF_W::new(self)
    }
    #[doc = "Bit 7 - UERIF"]
    #[inline(always)]
    #[must_use]
    pub fn uerif(&mut self) -> UERIF_W<7> {
        UERIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP1ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1isr](index.html) module"]
pub struct EP1ISR_SPEC;
impl crate::RegisterSpec for EP1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep1isr::R](R) reader structure"]
impl crate::Readable for EP1ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep1isr::W](W) writer structure"]
impl crate::Writable for EP1ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP1ISR to value 0"]
impl crate::Resettable for EP1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
