#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBEIEN` reader - TXBEIEN"]
pub type TXBEIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXBEIEN` writer - TXBEIEN"]
pub type TXBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXEIEN` reader - TXEIEN"]
pub type TXEIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEIEN` writer - TXEIEN"]
pub type TXEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXBNEIEN` reader - RXBNEIEN"]
pub type RXBNEIEN_R = crate::BitReader<bool>;
#[doc = "Field `RXBNEIEN` writer - RXBNEIEN"]
pub type RXBNEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `WCIEN` reader - WCIEN"]
pub type WCIEN_R = crate::BitReader<bool>;
#[doc = "Field `WCIEN` writer - WCIEN"]
pub type WCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ROIEN` reader - ROIEN"]
pub type ROIEN_R = crate::BitReader<bool>;
#[doc = "Field `ROIEN` writer - ROIEN"]
pub type ROIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `MFIEN` reader - MFIEN"]
pub type MFIEN_R = crate::BitReader<bool>;
#[doc = "Field `MFIEN` writer - MFIEN"]
pub type MFIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SAIEN` reader - SAIEN"]
pub type SAIEN_R = crate::BitReader<bool>;
#[doc = "Field `SAIEN` writer - SAIEN"]
pub type SAIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TOIEN` reader - TOIEN"]
pub type TOIEN_R = crate::BitReader<bool>;
#[doc = "Field `TOIEN` writer - TOIEN"]
pub type TOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    pub fn txbeien(&self) -> TXBEIEN_R {
        TXBEIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    pub fn txeien(&self) -> TXEIEN_R {
        TXEIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    pub fn rxbneien(&self) -> RXBNEIEN_R {
        RXBNEIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    pub fn wcien(&self) -> WCIEN_R {
        WCIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    pub fn mfien(&self) -> MFIEN_R {
        MFIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txbeien(&mut self) -> TXBEIEN_W<0> {
        TXBEIEN_W::new(self)
    }
    #[doc = "Bit 1 - TXEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn txeien(&mut self) -> TXEIEN_W<1> {
        TXEIEN_W::new(self)
    }
    #[doc = "Bit 2 - RXBNEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxbneien(&mut self) -> RXBNEIEN_W<2> {
        RXBNEIEN_W::new(self)
    }
    #[doc = "Bit 3 - WCIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wcien(&mut self) -> WCIEN_W<3> {
        WCIEN_W::new(self)
    }
    #[doc = "Bit 4 - ROIEN"]
    #[inline(always)]
    #[must_use]
    pub fn roien(&mut self) -> ROIEN_W<4> {
        ROIEN_W::new(self)
    }
    #[doc = "Bit 5 - MFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn mfien(&mut self) -> MFIEN_W<5> {
        MFIEN_W::new(self)
    }
    #[doc = "Bit 6 - SAIEN"]
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SAIEN_W<6> {
        SAIEN_W::new(self)
    }
    #[doc = "Bit 7 - TOIEN"]
    #[inline(always)]
    #[must_use]
    pub fn toien(&mut self) -> TOIEN_W<7> {
        TOIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
