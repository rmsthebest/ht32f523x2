#[doc = "Register `UART_URDLR` reader"]
pub type R = crate::R<UartUrdlrSpec>;
#[doc = "Register `UART_URDLR` writer"]
pub type W = crate::W<UartUrdlrSpec>;
#[doc = "Field `BRD` reader - BRD"]
pub type BrdR = crate::FieldReader<u16>;
#[doc = "Field `BRD` writer - BRD"]
pub type BrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    pub fn brd(&self) -> BrdR {
        BrdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BRD"]
    #[inline(always)]
    pub fn brd(&mut self) -> BrdW<'_, UartUrdlrSpec> {
        BrdW::new(self, 0)
    }
}
#[doc = "UART_URDLR\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_urdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_urdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUrdlrSpec;
impl crate::RegisterSpec for UartUrdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_urdlr::R`](R) reader structure"]
impl crate::Readable for UartUrdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_urdlr::W`](W) writer structure"]
impl crate::Writable for UartUrdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_URDLR to value 0"]
impl crate::Resettable for UartUrdlrSpec {}
