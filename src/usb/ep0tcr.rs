#[doc = "Register `EP0TCR` reader"]
pub struct R(crate::R<EP0TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP0TCR` writer"]
pub struct W(crate::W<EP0TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0TCR_SPEC>;
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
impl From<crate::W<EP0TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCNT` reader - TXCNT"]
pub type TXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCNT` writer - TXCNT"]
pub type TXCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0TCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RXCNT` reader - RXCNT"]
pub type RXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCNT` writer - RXCNT"]
pub type RXCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0TCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    #[must_use]
    pub fn txcnt(&mut self) -> TXCNT_W<0> {
        TXCNT_W::new(self)
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    #[must_use]
    pub fn rxcnt(&mut self) -> RXCNT_W<16> {
        RXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP0TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0tcr](index.html) module"]
pub struct EP0TCR_SPEC;
impl crate::RegisterSpec for EP0TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep0tcr::R](R) reader structure"]
impl crate::Readable for EP0TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep0tcr::W](W) writer structure"]
impl crate::Writable for EP0TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP0TCR to value 0"]
impl crate::Resettable for EP0TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
