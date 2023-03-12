#[doc = "Register `USART_IrDACR` reader"]
pub struct R(crate::R<USART_IR_DACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_IR_DACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_IR_DACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_IR_DACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_IrDACR` writer"]
pub struct W(crate::W<USART_IR_DACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_IR_DACR_SPEC>;
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
impl From<crate::W<USART_IR_DACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_IR_DACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IrDAEN` reader - IrDAEN"]
pub type IR_DAEN_R = crate::BitReader<bool>;
#[doc = "Field `IrDAEN` writer - IrDAEN"]
pub type IR_DAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `IrDALP` reader - IrDALP"]
pub type IR_DALP_R = crate::BitReader<bool>;
#[doc = "Field `IrDALP` writer - IrDALP"]
pub type IR_DALP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `TXSEL` reader - TXSEL"]
pub type TXSEL_R = crate::BitReader<bool>;
#[doc = "Field `TXSEL` writer - TXSEL"]
pub type TXSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `LB` reader - LB"]
pub type LB_R = crate::BitReader<bool>;
#[doc = "Field `LB` writer - LB"]
pub type LB_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - TXINV"]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - TXINV"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `RXINV` reader - RXINV"]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - RXINV"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USART_IR_DACR_SPEC, bool, O>;
#[doc = "Field `IrDAPSC` reader - IrDAPSC"]
pub type IR_DAPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IrDAPSC` writer - IrDAPSC"]
pub type IR_DAPSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USART_IR_DACR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    pub fn ir_daen(&self) -> IR_DAEN_R {
        IR_DAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    pub fn ir_dalp(&self) -> IR_DALP_R {
        IR_DALP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&self) -> TXSEL_R {
        TXSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    pub fn ir_dapsc(&self) -> IR_DAPSC_R {
        IR_DAPSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    #[must_use]
    pub fn ir_daen(&mut self) -> IR_DAEN_W<0> {
        IR_DAEN_W::new(self)
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    #[must_use]
    pub fn ir_dalp(&mut self) -> IR_DALP_W<1> {
        IR_DALP_W::new(self)
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    #[must_use]
    pub fn txsel(&mut self) -> TXSEL_W<2> {
        TXSEL_W::new(self)
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<3> {
        LB_W::new(self)
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<4> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<5> {
        RXINV_W::new(self)
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    #[must_use]
    pub fn ir_dapsc(&mut self) -> IR_DAPSC_W<8> {
        IR_DAPSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_IrDACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ir_dacr](index.html) module"]
pub struct USART_IR_DACR_SPEC;
impl crate::RegisterSpec for USART_IR_DACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_ir_dacr::R](R) reader structure"]
impl crate::Readable for USART_IR_DACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_ir_dacr::W](W) writer structure"]
impl crate::Writable for USART_IR_DACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_IrDACR to value 0"]
impl crate::Resettable for USART_IR_DACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
