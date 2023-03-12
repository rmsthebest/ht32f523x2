#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFTLS` reader - TXFTLS"]
pub type TXFTLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFTLS` writer - TXFTLS"]
pub type TXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXFTLS` reader - RXFTLS"]
pub type RXFTLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFTLS` writer - RXFTLS"]
pub type RXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXFRST` reader - TXFRST"]
pub type TXFRST_R = crate::BitReader<bool>;
#[doc = "Field `TXFRST` writer - TXFRST"]
pub type TXFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `RXFRST` reader - RXFRST"]
pub type RXFRST_R = crate::BitReader<bool>;
#[doc = "Field `RXFRST` writer - RXFRST"]
pub type RXFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&self) -> TXFTLS_R {
        TXFTLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&self) -> RXFTLS_R {
        RXFTLS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    pub fn txfrst(&self) -> TXFRST_R {
        TXFRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    pub fn rxfrst(&self) -> RXFRST_R {
        RXFRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn txftls(&mut self) -> TXFTLS_W<0> {
        TXFTLS_W::new(self)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn rxftls(&mut self) -> RXFTLS_W<4> {
        RXFTLS_W::new(self)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn txfrst(&mut self) -> TXFRST_W<8> {
        TXFRST_W::new(self)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrst(&mut self) -> RXFRST_W<9> {
        RXFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
