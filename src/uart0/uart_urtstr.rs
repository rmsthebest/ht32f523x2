#[doc = "Register `UART_URTSTR` reader"]
pub type R = crate::R<UartUrtstrSpec>;
#[doc = "Register `UART_URTSTR` writer"]
pub type W = crate::W<UartUrtstrSpec>;
#[doc = "Field `LBM` reader - LBM"]
pub type LbmR = crate::FieldReader;
#[doc = "Field `LBM` writer - LBM"]
pub type LbmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<UartUrtstrSpec> {
        LbmW::new(self, 0)
    }
}
#[doc = "UART_URTSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urtstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urtstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUrtstrSpec;
impl crate::RegisterSpec for UartUrtstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_urtstr::R`](R) reader structure"]
impl crate::Readable for UartUrtstrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_urtstr::W`](W) writer structure"]
impl crate::Writable for UartUrtstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_URTSTR to value 0"]
impl crate::Resettable for UartUrtstrSpec {
    const RESET_VALUE: u32 = 0;
}
