#[doc = "Register `LPCR` reader"]
pub struct R(crate::R<LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCR` writer"]
pub struct W(crate::W<LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCR_SPEC>;
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
impl From<crate::W<LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKISO` reader - BKISO"]
pub type BKISO_R = crate::BitReader<bool>;
#[doc = "Field `BKISO` writer - BKISO"]
pub type BKISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `USBSLEEP` reader - USBSLEEP"]
pub type USBSLEEP_R = crate::BitReader<bool>;
#[doc = "Field `USBSLEEP` writer - USBSLEEP"]
pub type USBSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    pub fn bkiso(&self) -> BKISO_R {
        BKISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&self) -> USBSLEEP_R {
        USBSLEEP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    #[must_use]
    pub fn bkiso(&mut self) -> BKISO_W<0> {
        BKISO_W::new(self)
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn usbsleep(&mut self) -> USBSLEEP_W<8> {
        USBSLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](index.html) module"]
pub struct LPCR_SPEC;
impl crate::RegisterSpec for LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcr::R](R) reader structure"]
impl crate::Readable for LPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpcr::W](W) writer structure"]
impl crate::Writable for LPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCR to value 0"]
impl crate::Resettable for LPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
