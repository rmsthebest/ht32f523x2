#[doc = "Register `USART_IrDACR` reader"]
pub type R = crate::R<UsartIrDacrSpec>;
#[doc = "Register `USART_IrDACR` writer"]
pub type W = crate::W<UsartIrDacrSpec>;
#[doc = "Field `IrDAEN` reader - IrDAEN"]
pub type IrDaenR = crate::BitReader;
#[doc = "Field `IrDAEN` writer - IrDAEN"]
pub type IrDaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IrDALP` reader - IrDALP"]
pub type IrDalpR = crate::BitReader;
#[doc = "Field `IrDALP` writer - IrDALP"]
pub type IrDalpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEL` reader - TXSEL"]
pub type TxselR = crate::BitReader;
#[doc = "Field `TXSEL` writer - TXSEL"]
pub type TxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LB` reader - LB"]
pub type LbR = crate::BitReader;
#[doc = "Field `LB` writer - LB"]
pub type LbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TXINV"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TXINV"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RXINV"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - RXINV"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IrDAPSC` reader - IrDAPSC"]
pub type IrDapscR = crate::FieldReader;
#[doc = "Field `IrDAPSC` writer - IrDAPSC"]
pub type IrDapscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    pub fn ir_daen(&self) -> IrDaenR {
        IrDaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    pub fn ir_dalp(&self) -> IrDalpR {
        IrDalpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&self) -> TxselR {
        TxselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    pub fn lb(&self) -> LbR {
        LbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    pub fn ir_dapsc(&self) -> IrDapscR {
        IrDapscR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDAEN"]
    #[inline(always)]
    #[must_use]
    pub fn ir_daen(&mut self) -> IrDaenW<UsartIrDacrSpec> {
        IrDaenW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDALP"]
    #[inline(always)]
    #[must_use]
    pub fn ir_dalp(&mut self) -> IrDalpW<UsartIrDacrSpec> {
        IrDalpW::new(self, 1)
    }
    #[doc = "Bit 2 - TXSEL"]
    #[inline(always)]
    #[must_use]
    pub fn txsel(&mut self) -> TxselW<UsartIrDacrSpec> {
        TxselW::new(self, 2)
    }
    #[doc = "Bit 3 - LB"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LbW<UsartIrDacrSpec> {
        LbW::new(self, 3)
    }
    #[doc = "Bit 4 - TXINV"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<UsartIrDacrSpec> {
        TxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - RXINV"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<UsartIrDacrSpec> {
        RxinvW::new(self, 5)
    }
    #[doc = "Bits 8:15 - IrDAPSC"]
    #[inline(always)]
    #[must_use]
    pub fn ir_dapsc(&mut self) -> IrDapscW<UsartIrDacrSpec> {
        IrDapscW::new(self, 8)
    }
}
#[doc = "USART_IrDACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_ir_dacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_ir_dacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartIrDacrSpec;
impl crate::RegisterSpec for UsartIrDacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_ir_dacr::R`](R) reader structure"]
impl crate::Readable for UsartIrDacrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_ir_dacr::W`](W) writer structure"]
impl crate::Writable for UsartIrDacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_IrDACR to value 0"]
impl crate::Resettable for UsartIrDacrSpec {
    const RESET_VALUE: u32 = 0;
}
