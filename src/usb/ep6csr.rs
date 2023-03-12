#[doc = "Register `EP6CSR` reader"]
pub struct R(crate::R<EP6CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP6CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP6CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP6CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP6CSR` writer"]
pub struct W(crate::W<EP6CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP6CSR_SPEC>;
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
impl From<crate::W<EP6CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP6CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGTX` reader - DTGTX"]
pub type DTGTX_R = crate::BitReader<bool>;
#[doc = "Field `DTGTX` writer - DTGTX"]
pub type DTGTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `NAKTX` reader - NAKTX"]
pub type NAKTX_R = crate::BitReader<bool>;
#[doc = "Field `NAKTX` writer - NAKTX"]
pub type NAKTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `STLTX` reader - STLTX"]
pub type STLTX_R = crate::BitReader<bool>;
#[doc = "Field `STLTX` writer - STLTX"]
pub type STLTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `DTGRX` reader - DTGRX"]
pub type DTGRX_R = crate::BitReader<bool>;
#[doc = "Field `DTGRX` writer - DTGRX"]
pub type DTGRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `NAKRX` reader - NAKRX"]
pub type NAKRX_R = crate::BitReader<bool>;
#[doc = "Field `NAKRX` writer - NAKRX"]
pub type NAKRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `STLRX` reader - STLRX"]
pub type STLRX_R = crate::BitReader<bool>;
#[doc = "Field `STLRX` writer - STLRX"]
pub type STLRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `MDBTG` reader - MDBTG"]
pub type MDBTG_R = crate::BitReader<bool>;
#[doc = "Field `MDBTG` writer - MDBTG"]
pub type MDBTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
#[doc = "Field `UDBTG` reader - UDBTG"]
pub type UDBTG_R = crate::BitReader<bool>;
#[doc = "Field `UDBTG` writer - UDBTG"]
pub type UDBTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&self) -> DTGTX_R {
        DTGTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&self) -> NAKTX_R {
        NAKTX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&self) -> STLTX_R {
        STLTX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&self) -> DTGRX_R {
        DTGRX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&self) -> NAKRX_R {
        NAKRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&self) -> STLRX_R {
        STLRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    pub fn mdbtg(&self) -> MDBTG_R {
        MDBTG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    pub fn udbtg(&self) -> UDBTG_R {
        UDBTG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    #[must_use]
    pub fn dtgtx(&mut self) -> DTGTX_W<0> {
        DTGTX_W::new(self)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    #[must_use]
    pub fn naktx(&mut self) -> NAKTX_W<1> {
        NAKTX_W::new(self)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    #[must_use]
    pub fn stltx(&mut self) -> STLTX_W<2> {
        STLTX_W::new(self)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    #[must_use]
    pub fn dtgrx(&mut self) -> DTGRX_W<3> {
        DTGRX_W::new(self)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    #[must_use]
    pub fn nakrx(&mut self) -> NAKRX_W<4> {
        NAKRX_W::new(self)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    #[must_use]
    pub fn stlrx(&mut self) -> STLRX_W<5> {
        STLRX_W::new(self)
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    #[must_use]
    pub fn mdbtg(&mut self) -> MDBTG_W<6> {
        MDBTG_W::new(self)
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    #[must_use]
    pub fn udbtg(&mut self) -> UDBTG_W<7> {
        UDBTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP6CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6csr](index.html) module"]
pub struct EP6CSR_SPEC;
impl crate::RegisterSpec for EP6CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep6csr::R](R) reader structure"]
impl crate::Readable for EP6CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep6csr::W](W) writer structure"]
impl crate::Writable for EP6CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP6CSR to value 0"]
impl crate::Resettable for EP6CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
