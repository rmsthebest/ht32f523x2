#[doc = "Register `UART_URCR` reader"]
pub struct R(crate::R<UART_URCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_URCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_URCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_URCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_URCR` writer"]
pub struct W(crate::W<UART_URCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_URCR_SPEC>;
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
impl From<crate::W<UART_URCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_URCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRSM` reader - TRSM"]
pub type TRSM_R = crate::BitReader<bool>;
#[doc = "Field `TRSM` writer - TRSM"]
pub type TRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `URTXEN` reader - URTXEN"]
pub type URTXEN_R = crate::BitReader<bool>;
#[doc = "Field `URTXEN` writer - URTXEN"]
pub type URTXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `URRXEN` reader - URRXEN"]
pub type URRXEN_R = crate::BitReader<bool>;
#[doc = "Field `URRXEN` writer - URRXEN"]
pub type URRXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `WLS` reader - WLS"]
pub type WLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLS` writer - WLS"]
pub type WLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_URCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NSB` reader - NSB"]
pub type NSB_R = crate::BitReader<bool>;
#[doc = "Field `NSB` writer - NSB"]
pub type NSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `PBE` reader - PBE"]
pub type PBE_R = crate::BitReader<bool>;
#[doc = "Field `PBE` writer - PBE"]
pub type PBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `EPE` reader - EPE"]
pub type EPE_R = crate::BitReader<bool>;
#[doc = "Field `EPE` writer - EPE"]
pub type EPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `SPE` reader - SPE"]
pub type SPE_R = crate::BitReader<bool>;
#[doc = "Field `SPE` writer - SPE"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
#[doc = "Field `BCB` reader - BCB"]
pub type BCB_R = crate::BitReader<bool>;
#[doc = "Field `BCB` writer - BCB"]
pub type BCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_URCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&self) -> TRSM_R {
        TRSM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    pub fn urtxen(&self) -> URTXEN_R {
        URTXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    pub fn urrxen(&self) -> URRXEN_R {
        URRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    #[must_use]
    pub fn trsm(&mut self) -> TRSM_W<2> {
        TRSM_W::new(self)
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urtxen(&mut self) -> URTXEN_W<4> {
        URTXEN_W::new(self)
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urrxen(&mut self) -> URRXEN_W<5> {
        URRXEN_W::new(self)
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<6> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<7> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WLS_W<8> {
        WLS_W::new(self)
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    #[must_use]
    pub fn nsb(&mut self) -> NSB_W<10> {
        NSB_W::new(self)
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    #[must_use]
    pub fn pbe(&mut self) -> PBE_W<11> {
        PBE_W::new(self)
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<12> {
        EPE_W::new(self)
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<13> {
        SPE_W::new(self)
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    #[must_use]
    pub fn bcb(&mut self) -> BCB_W<14> {
        BCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_URCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urcr](index.html) module"]
pub struct UART_URCR_SPEC;
impl crate::RegisterSpec for UART_URCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_urcr::R](R) reader structure"]
impl crate::Readable for UART_URCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_urcr::W](W) writer structure"]
impl crate::Writable for UART_URCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_URCR to value 0"]
impl crate::Resettable for UART_URCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
