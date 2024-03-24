#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `TXFTLS` reader - TXFTLS"]
pub type TxftlsR = crate::FieldReader;
#[doc = "Field `TXFTLS` writer - TXFTLS"]
pub type TxftlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXFTLS` reader - RXFTLS"]
pub type RxftlsR = crate::FieldReader;
#[doc = "Field `RXFTLS` writer - RXFTLS"]
pub type RxftlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXFRST` reader - TXFRST"]
pub type TxfrstR = crate::BitReader;
#[doc = "Field `TXFRST` writer - TXFRST"]
pub type TxfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRST` reader - RXFRST"]
pub type RxfrstR = crate::BitReader;
#[doc = "Field `RXFRST` writer - RXFRST"]
pub type RxfrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&self) -> TxftlsR {
        TxftlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&self) -> RxftlsR {
        RxftlsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    pub fn txfrst(&self) -> TxfrstR {
        TxfrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    pub fn rxfrst(&self) -> RxfrstR {
        RxfrstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn txftls(&mut self) -> TxftlsW<FcrSpec> {
        TxftlsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn rxftls(&mut self) -> RxftlsW<FcrSpec> {
        RxftlsW::new(self, 4)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn txfrst(&mut self) -> TxfrstW<FcrSpec> {
        TxfrstW::new(self, 8)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrst(&mut self) -> RxfrstW<FcrSpec> {
        RxfrstW::new(self, 9)
    }
}
#[doc = "FCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u32 = 0;
}
