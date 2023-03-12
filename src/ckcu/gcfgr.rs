#[doc = "Register `GCFGR` reader"]
pub struct R(crate::R<GCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCFGR` writer"]
pub struct W(crate::W<GCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCFGR_SPEC>;
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
impl From<crate::W<GCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CKOUTSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CKOUTSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLSRC` reader - PLLSRC"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - PLLSRC"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCFGR_SPEC, bool, O>;
#[doc = "Field `CKREFPRE` reader - CKREFPRE"]
pub type CKREFPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKREFPRE` writer - CKREFPRE"]
pub type CKREFPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `USBPRE` reader - USBPRE"]
pub type USBPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPRE` writer - USBPRE"]
pub type USBPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPMOD` reader - LPMOD"]
pub type LPMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMOD` writer - LPMOD"]
pub type LPMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    pub fn ckrefpre(&self) -> CKREFPRE_R {
        CKREFPRE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&self) -> LPMOD_R {
        LPMOD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<0> {
        CKOUTSRC_W::new(self)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<8> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    #[must_use]
    pub fn ckrefpre(&mut self) -> CKREFPRE_W<11> {
        CKREFPRE_W::new(self)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    #[must_use]
    pub fn usbpre(&mut self) -> USBPRE_W<22> {
        USBPRE_W::new(self)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn lpmod(&mut self) -> LPMOD_W<29> {
        LPMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfgr](index.html) module"]
pub struct GCFGR_SPEC;
impl crate::RegisterSpec for GCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcfgr::R](R) reader structure"]
impl crate::Readable for GCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcfgr::W](W) writer structure"]
impl crate::Writable for GCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCFGR to value 0"]
impl crate::Resettable for GCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
