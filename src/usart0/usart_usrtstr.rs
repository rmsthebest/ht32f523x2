#[doc = "Register `USART_USRTSTR` reader"]
pub type R = crate::R<UsartUsrtstrSpec>;
#[doc = "Register `USART_USRTSTR` writer"]
pub type W = crate::W<UsartUsrtstrSpec>;
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
    pub fn lbm(&mut self) -> LbmW<'_, UsartUsrtstrSpec> {
        LbmW::new(self, 0)
    }
}
#[doc = "USART_USRTSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`usart_usrtstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_usrtstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrtstrSpec;
impl crate::RegisterSpec for UsartUsrtstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrtstr::R`](R) reader structure"]
impl crate::Readable for UsartUsrtstrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrtstr::W`](W) writer structure"]
impl crate::Writable for UsartUsrtstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART_USRTSTR to value 0"]
impl crate::Resettable for UsartUsrtstrSpec {}
