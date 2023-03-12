#[doc = "Register `UART_URIER` reader"]
pub struct R(crate::R<UART_URIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_URIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_URIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_URIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_URIER` writer"]
pub struct W(crate::W<UART_URIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_URIER_SPEC>;
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
impl From<crate::W<UART_URIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_URIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDRIE` reader - RXDRIE"]
pub type RXDRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDRIE` writer - RXDRIE"]
pub type RXDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `TXDEIE` reader - TXDEIE"]
pub type TXDEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDEIE` writer - TXDEIE"]
pub type TXDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `TXCIE` reader - TXCIE"]
pub type TXCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE` writer - TXCIE"]
pub type TXCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `OEIE` reader - OEIE"]
pub type OEIE_R = crate::BitReader<bool>;
#[doc = "Field `OEIE` writer - OEIE"]
pub type OEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `PEIE` reader - PEIE"]
pub type PEIE_R = crate::BitReader<bool>;
#[doc = "Field `PEIE` writer - PEIE"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `FEIE` reader - FEIE"]
pub type FEIE_R = crate::BitReader<bool>;
#[doc = "Field `FEIE` writer - FEIE"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
#[doc = "Field `BIE` reader - BIE"]
pub type BIE_R = crate::BitReader<bool>;
#[doc = "Field `BIE` writer - BIE"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RXDRIE_R {
        RXDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TXDEIE_R {
        TXDEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    pub fn oeie(&self) -> OEIE_R {
        OEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrie(&mut self) -> RXDRIE_W<0> {
        RXDRIE_W::new(self)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TXDEIE_W<1> {
        TXDEIE_W::new(self)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<2> {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 3 - OEIE"]
    #[inline(always)]
    #[must_use]
    pub fn oeie(&mut self) -> OEIE_W<3> {
        OEIE_W::new(self)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<4> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<5> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<6> {
        BIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_URIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urier](index.html) module"]
pub struct UART_URIER_SPEC;
impl crate::RegisterSpec for UART_URIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_urier::R](R) reader structure"]
impl crate::Readable for UART_URIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_urier::W](W) writer structure"]
impl crate::Writable for UART_URIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_URIER to value 0"]
impl crate::Resettable for UART_URIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
