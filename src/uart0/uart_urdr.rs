#[doc = "Register `UART_URDR` reader"]
pub type R = crate::R<UartUrdrSpec>;
#[doc = "Register `UART_URDR` writer"]
pub type W = crate::W<UartUrdrSpec>;
#[doc = "Field `DB` reader - DB"]
pub type DbR = crate::FieldReader<u16>;
#[doc = "Field `DB` writer - DB"]
pub type DbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    pub fn db(&self) -> DbR {
        DbR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DB"]
    #[inline(always)]
    #[must_use]
    pub fn db(&mut self) -> DbW<UartUrdrSpec> {
        DbW::new(self, 0)
    }
}
#[doc = "UART_URDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUrdrSpec;
impl crate::RegisterSpec for UartUrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_urdr::R`](R) reader structure"]
impl crate::Readable for UartUrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_urdr::W`](W) writer structure"]
impl crate::Writable for UartUrdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_URDR to value 0"]
impl crate::Resettable for UartUrdrSpec {
    const RESET_VALUE: u32 = 0;
}
