#[doc = "Register `USART_USRCR` reader"]
pub type R = crate::R<UsartUsrcrSpec>;
#[doc = "Register `USART_USRCR` writer"]
pub type W = crate::W<UsartUsrcrSpec>;
#[doc = "Field `MODE` reader - MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRSM` reader - TRSM"]
pub type TrsmR = crate::BitReader;
#[doc = "Field `TRSM` writer - TRSM"]
pub type TrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFCEN` reader - HFCEN"]
pub type HfcenR = crate::BitReader;
#[doc = "Field `HFCEN` writer - HFCEN"]
pub type HfcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URTXEN` reader - URTXEN"]
pub type UrtxenR = crate::BitReader;
#[doc = "Field `URTXEN` writer - URTXEN"]
pub type UrtxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URRXEN` reader - URRXEN"]
pub type UrrxenR = crate::BitReader;
#[doc = "Field `URRXEN` writer - URRXEN"]
pub type UrrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLS` reader - WLS"]
pub type WlsR = crate::FieldReader;
#[doc = "Field `WLS` writer - WLS"]
pub type WlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSB` reader - NSB"]
pub type NsbR = crate::BitReader;
#[doc = "Field `NSB` writer - NSB"]
pub type NsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBE` reader - PBE"]
pub type PbeR = crate::BitReader;
#[doc = "Field `PBE` writer - PBE"]
pub type PbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - EPE"]
pub type EpeR = crate::BitReader;
#[doc = "Field `EPE` writer - EPE"]
pub type EpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPE` reader - SPE"]
pub type SpeR = crate::BitReader;
#[doc = "Field `SPE` writer - SPE"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCB` reader - BCB"]
pub type BcbR = crate::BitReader;
#[doc = "Field `BCB` writer - BCB"]
pub type BcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - RTS"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - RTS"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&self) -> TrsmR {
        TrsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFCEN"]
    #[inline(always)]
    pub fn hfcen(&self) -> HfcenR {
        HfcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    pub fn urtxen(&self) -> UrtxenR {
        UrtxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    pub fn urrxen(&self) -> UrrxenR {
        UrrxenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    pub fn wls(&self) -> WlsR {
        WlsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    pub fn nsb(&self) -> NsbR {
        NsbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PbeR {
        PbeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    pub fn bcb(&self) -> BcbR {
        BcbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<UsartUsrcrSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    #[must_use]
    pub fn trsm(&mut self) -> TrsmW<UsartUsrcrSpec> {
        TrsmW::new(self, 2)
    }
    #[doc = "Bit 3 - HFCEN"]
    #[inline(always)]
    #[must_use]
    pub fn hfcen(&mut self) -> HfcenW<UsartUsrcrSpec> {
        HfcenW::new(self, 3)
    }
    #[doc = "Bit 4 - URTXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urtxen(&mut self) -> UrtxenW<UsartUsrcrSpec> {
        UrtxenW::new(self, 4)
    }
    #[doc = "Bit 5 - URRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urrxen(&mut self) -> UrrxenW<UsartUsrcrSpec> {
        UrrxenW::new(self, 5)
    }
    #[doc = "Bit 6 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<UsartUsrcrSpec> {
        TxdmaenW::new(self, 6)
    }
    #[doc = "Bit 7 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<UsartUsrcrSpec> {
        RxdmaenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - WLS"]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WlsW<UsartUsrcrSpec> {
        WlsW::new(self, 8)
    }
    #[doc = "Bit 10 - NSB"]
    #[inline(always)]
    #[must_use]
    pub fn nsb(&mut self) -> NsbW<UsartUsrcrSpec> {
        NsbW::new(self, 10)
    }
    #[doc = "Bit 11 - PBE"]
    #[inline(always)]
    #[must_use]
    pub fn pbe(&mut self) -> PbeW<UsartUsrcrSpec> {
        PbeW::new(self, 11)
    }
    #[doc = "Bit 12 - EPE"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EpeW<UsartUsrcrSpec> {
        EpeW::new(self, 12)
    }
    #[doc = "Bit 13 - SPE"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SpeW<UsartUsrcrSpec> {
        SpeW::new(self, 13)
    }
    #[doc = "Bit 14 - BCB"]
    #[inline(always)]
    #[must_use]
    pub fn bcb(&mut self) -> BcbW<UsartUsrcrSpec> {
        BcbW::new(self, 14)
    }
    #[doc = "Bit 15 - RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<UsartUsrcrSpec> {
        RtsW::new(self, 15)
    }
}
#[doc = "USART_USRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrcrSpec;
impl crate::RegisterSpec for UsartUsrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrcr::R`](R) reader structure"]
impl crate::Readable for UsartUsrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrcr::W`](W) writer structure"]
impl crate::Writable for UsartUsrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRCR to value 0"]
impl crate::Resettable for UsartUsrcrSpec {
    const RESET_VALUE: u32 = 0;
}
