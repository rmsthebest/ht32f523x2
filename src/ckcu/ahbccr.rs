#[doc = "Register `AHBCCR` reader"]
pub struct R(crate::R<AHBCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCCR` writer"]
pub struct W(crate::W<AHBCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCCR_SPEC>;
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
impl From<crate::W<AHBCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - FMCEN"]
pub type FMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCEN` writer - FMCEN"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `SRAMEN` reader - SRAMEN"]
pub type SRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAMEN` writer - SRAMEN"]
pub type SRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `PDMAEN` reader - PDMAEN"]
pub type PDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `PDMAEN` writer - PDMAEN"]
pub type PDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `BMEN` reader - BMEN"]
pub type BMEN_R = crate::BitReader<bool>;
#[doc = "Field `BMEN` writer - BMEN"]
pub type BMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `APBEN` reader - APBEN"]
pub type APBEN_R = crate::BitReader<bool>;
#[doc = "Field `APBEN` writer - APBEN"]
pub type APBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - USBEN"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USBEN"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `CKREFEN` reader - CKREFEN"]
pub type CKREFEN_R = crate::BitReader<bool>;
#[doc = "Field `CKREFEN` writer - CKREFEN"]
pub type CKREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `EBIEN` reader - EBIEN"]
pub type EBIEN_R = crate::BitReader<bool>;
#[doc = "Field `EBIEN` writer - EBIEN"]
pub type EBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - CRCEN"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRCEN"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `PAEN` reader - PAEN"]
pub type PAEN_R = crate::BitReader<bool>;
#[doc = "Field `PAEN` writer - PAEN"]
pub type PAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `PBEN` reader - PBEN"]
pub type PBEN_R = crate::BitReader<bool>;
#[doc = "Field `PBEN` writer - PBEN"]
pub type PBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `PCEN` reader - PCEN"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - PCEN"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
#[doc = "Field `PDEN` reader - PDEN"]
pub type PDEN_R = crate::BitReader<bool>;
#[doc = "Field `PDEN` writer - PDEN"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    pub fn apben(&self) -> APBEN_R {
        APBEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    pub fn ckrefen(&self) -> CKREFEN_R {
        CKREFEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    pub fn ebien(&self) -> EBIEN_R {
        EBIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pdmaen(&mut self) -> PDMAEN_W<4> {
        PDMAEN_W::new(self)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    #[must_use]
    pub fn bmen(&mut self) -> BMEN_W<5> {
        BMEN_W::new(self)
    }
    #[doc = "Bit 6 - APBEN"]
    #[inline(always)]
    #[must_use]
    pub fn apben(&mut self) -> APBEN_W<6> {
        APBEN_W::new(self)
    }
    #[doc = "Bit 10 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<10> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 11 - CKREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckrefen(&mut self) -> CKREFEN_W<11> {
        CKREFEN_W::new(self)
    }
    #[doc = "Bit 12 - EBIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ebien(&mut self) -> EBIEN_W<12> {
        EBIEN_W::new(self)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<16> {
        PAEN_W::new(self)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<17> {
        PBEN_W::new(self)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<18> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<19> {
        PDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHBCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbccr](index.html) module"]
pub struct AHBCCR_SPEC;
impl crate::RegisterSpec for AHBCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbccr::R](R) reader structure"]
impl crate::Readable for AHBCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbccr::W](W) writer structure"]
impl crate::Writable for AHBCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBCCR to value 0"]
impl crate::Resettable for AHBCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
