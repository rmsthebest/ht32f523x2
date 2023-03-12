#[doc = "Register `USART_RS485CR` reader"]
pub struct R(crate::R<USART_RS485CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_RS485CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_RS485CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_RS485CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_RS485CR` writer"]
pub struct W(crate::W<USART_RS485CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_RS485CR_SPEC>;
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
impl From<crate::W<USART_RS485CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_RS485CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXENP` reader - TXENP"]
pub type TXENP_R = crate::BitReader<bool>;
#[doc = "Field `TXENP` writer - TXENP"]
pub type TXENP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_RS485CR_SPEC, bool, O>;
#[doc = "Field `RSNMM` reader - RSNMM"]
pub type RSNMM_R = crate::BitReader<bool>;
#[doc = "Field `RSNMM` writer - RSNMM"]
pub type RSNMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_RS485CR_SPEC, bool, O>;
#[doc = "Field `RSAAD` reader - RSAAD"]
pub type RSAAD_R = crate::BitReader<bool>;
#[doc = "Field `RSAAD` writer - RSAAD"]
pub type RSAAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_RS485CR_SPEC, bool, O>;
#[doc = "Field `ADDMATCH` reader - ADDMATCH"]
pub type ADDMATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDMATCH` writer - ADDMATCH"]
pub type ADDMATCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USART_RS485CR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    pub fn txenp(&self) -> TXENP_R {
        TXENP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    pub fn rsnmm(&self) -> RSNMM_R {
        RSNMM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    pub fn rsaad(&self) -> RSAAD_R {
        RSAAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    pub fn addmatch(&self) -> ADDMATCH_R {
        ADDMATCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    #[must_use]
    pub fn txenp(&mut self) -> TXENP_W<0> {
        TXENP_W::new(self)
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    #[must_use]
    pub fn rsnmm(&mut self) -> RSNMM_W<1> {
        RSNMM_W::new(self)
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    #[must_use]
    pub fn rsaad(&mut self) -> RSAAD_W<2> {
        RSAAD_W::new(self)
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    #[must_use]
    pub fn addmatch(&mut self) -> ADDMATCH_W<8> {
        ADDMATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_RS485CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_rs485cr](index.html) module"]
pub struct USART_RS485CR_SPEC;
impl crate::RegisterSpec for USART_RS485CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_rs485cr::R](R) reader structure"]
impl crate::Readable for USART_RS485CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_rs485cr::W](W) writer structure"]
impl crate::Writable for USART_RS485CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_RS485CR to value 0"]
impl crate::Resettable for USART_RS485CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
