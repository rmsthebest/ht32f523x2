#[doc = "Register `USART_USRFCR` reader"]
pub type R = crate::R<UsartUsrfcrSpec>;
#[doc = "Register `USART_USRFCR` writer"]
pub type W = crate::W<UsartUsrfcrSpec>;
#[doc = "Field `TXR` reader - TXR"]
pub type TxrR = crate::BitReader;
#[doc = "Field `TXR` writer - TXR"]
pub type TxrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXR` reader - RXR"]
pub type RxrR = crate::BitReader;
#[doc = "Field `RXR` writer - RXR"]
pub type RxrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTL` reader - TXTL"]
pub type TxtlR = crate::FieldReader;
#[doc = "Field `TXTL` writer - TXTL"]
pub type TxtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTL` reader - RXTL"]
pub type RxtlR = crate::FieldReader;
#[doc = "Field `RXTL` writer - RXTL"]
pub type RxtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXFS` reader - TXFS"]
pub type TxfsR = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RxfsR = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    pub fn txr(&self) -> TxrR {
        TxrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    pub fn rxr(&self) -> RxrR {
        RxrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    pub fn txtl(&self) -> TxtlR {
        TxtlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    pub fn rxtl(&self) -> RxtlR {
        RxtlR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TxfsR {
        TxfsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RxfsR {
        RxfsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    #[must_use]
    pub fn txr(&mut self) -> TxrW<UsartUsrfcrSpec> {
        TxrW::new(self, 0)
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    #[must_use]
    pub fn rxr(&mut self) -> RxrW<UsartUsrfcrSpec> {
        RxrW::new(self, 1)
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    #[must_use]
    pub fn txtl(&mut self) -> TxtlW<UsartUsrfcrSpec> {
        TxtlW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    #[must_use]
    pub fn rxtl(&mut self) -> RxtlW<UsartUsrfcrSpec> {
        RxtlW::new(self, 6)
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    #[must_use]
    pub fn txfs(&mut self) -> TxfsW<UsartUsrfcrSpec> {
        TxfsW::new(self, 16)
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfs(&mut self) -> RxfsW<UsartUsrfcrSpec> {
        RxfsW::new(self, 24)
    }
}
#[doc = "USART_USRFCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrfcrSpec;
impl crate::RegisterSpec for UsartUsrfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrfcr::R`](R) reader structure"]
impl crate::Readable for UsartUsrfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrfcr::W`](W) writer structure"]
impl crate::Writable for UsartUsrfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRFCR to value 0"]
impl crate::Resettable for UsartUsrfcrSpec {
    const RESET_VALUE: u32 = 0;
}
