#[doc = "Register `EP0TCR` reader"]
pub type R = crate::R<Ep0tcrSpec>;
#[doc = "Register `EP0TCR` writer"]
pub type W = crate::W<Ep0tcrSpec>;
#[doc = "Field `TXCNT` reader - TXCNT"]
pub type TxcntR = crate::FieldReader;
#[doc = "Field `TXCNT` writer - TXCNT"]
pub type TxcntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RXCNT` reader - RXCNT"]
pub type RxcntR = crate::FieldReader;
#[doc = "Field `RXCNT` writer - RXCNT"]
pub type RxcntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    pub fn txcnt(&self) -> TxcntR {
        TxcntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RxcntR {
        RxcntR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TXCNT"]
    #[inline(always)]
    #[must_use]
    pub fn txcnt(&mut self) -> TxcntW<Ep0tcrSpec> {
        TxcntW::new(self, 0)
    }
    #[doc = "Bits 16:22 - RXCNT"]
    #[inline(always)]
    #[must_use]
    pub fn rxcnt(&mut self) -> RxcntW<Ep0tcrSpec> {
        RxcntW::new(self, 16)
    }
}
#[doc = "EP0TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0tcrSpec;
impl crate::RegisterSpec for Ep0tcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0tcr::R`](R) reader structure"]
impl crate::Readable for Ep0tcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0tcr::W`](W) writer structure"]
impl crate::Writable for Ep0tcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0TCR to value 0"]
impl crate::Resettable for Ep0tcrSpec {
    const RESET_VALUE: u32 = 0;
}
