#[doc = "Register `USART_USRDLR` reader"]
pub type R = crate::R<UsartUsrdlrSpec>;
#[doc = "Register `USART_USRDLR` writer"]
pub type W = crate::W<UsartUsrdlrSpec>;
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
    pub fn brd(&mut self) -> BrdW<'_, UsartUsrdlrSpec> {
        BrdW::new(self, 0)
    }
}
#[doc = "USART_USRDLR\n\nYou can [`read`](crate::Reg::read) this register and get [`usart_usrdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_usrdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrdlrSpec;
impl crate::RegisterSpec for UsartUsrdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrdlr::R`](R) reader structure"]
impl crate::Readable for UsartUsrdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrdlr::W`](W) writer structure"]
impl crate::Writable for UsartUsrdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART_USRDLR to value 0"]
impl crate::Resettable for UsartUsrdlrSpec {}
