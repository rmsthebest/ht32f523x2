#[doc = "Register `GCCR` reader"]
pub struct R(crate::R<GCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCR` writer"]
pub struct W(crate::W<GCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCR_SPEC>;
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
impl From<crate::W<GCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - SW"]
pub type SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` writer - SW"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HSEGAIN` reader - HSEGAIN"]
pub type HSEGAIN_R = crate::BitReader<bool>;
#[doc = "Field `HSEGAIN` writer - HSEGAIN"]
pub type HSEGAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
#[doc = "Field `HSEEN` reader - HSEEN"]
pub type HSEEN_R = crate::BitReader<bool>;
#[doc = "Field `HSEEN` writer - HSEEN"]
pub type HSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
#[doc = "Field `HSIEN` reader - HSIEN"]
pub type HSIEN_R = crate::BitReader<bool>;
#[doc = "Field `HSIEN` writer - HSIEN"]
pub type HSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
#[doc = "Field `CKMEN` reader - CKMEN"]
pub type CKMEN_R = crate::BitReader<bool>;
#[doc = "Field `CKMEN` writer - CKMEN"]
pub type CKMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
#[doc = "Field `PSRCEN` reader - PSRCEN"]
pub type PSRCEN_R = crate::BitReader<bool>;
#[doc = "Field `PSRCEN` writer - PSRCEN"]
pub type PSRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    pub fn hsegain(&self) -> HSEGAIN_R {
        HSEGAIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    pub fn psrcen(&self) -> PSRCEN_R {
        PSRCEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    #[must_use]
    pub fn hsegain(&mut self) -> HSEGAIN_W<8> {
        HSEGAIN_W::new(self)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<9> {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn hseen(&mut self) -> HSEEN_W<10> {
        HSEEN_W::new(self)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsien(&mut self) -> HSIEN_W<11> {
        HSIEN_W::new(self)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CKMEN_W<16> {
        CKMEN_W::new(self)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn psrcen(&mut self) -> PSRCEN_W<17> {
        PSRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gccr](index.html) module"]
pub struct GCCR_SPEC;
impl crate::RegisterSpec for GCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gccr::R](R) reader structure"]
impl crate::Readable for GCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gccr::W](W) writer structure"]
impl crate::Writable for GCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCR to value 0"]
impl crate::Resettable for GCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
