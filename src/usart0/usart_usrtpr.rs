#[doc = "Register `USART_USRTPR` reader"]
pub type R = crate::R<UsartUsrtprSpec>;
#[doc = "Register `USART_USRTPR` writer"]
pub type W = crate::W<UsartUsrtprSpec>;
#[doc = "Field `RXTOC` reader - RXTOC"]
pub type RxtocR = crate::FieldReader;
#[doc = "Field `RXTOC` writer - RXTOC"]
pub type RxtocW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RXTOEN` reader - RXTOEN"]
pub type RxtoenR = crate::BitReader;
#[doc = "Field `RXTOEN` writer - RXTOEN"]
pub type RxtoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` reader - TG"]
pub type TgR = crate::FieldReader;
#[doc = "Field `TG` writer - TG"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    pub fn rxtoc(&self) -> RxtocR {
        RxtocR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    pub fn rxtoen(&self) -> RxtoenR {
        RxtoenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RXTOC"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoc(&mut self) -> RxtocW<UsartUsrtprSpec> {
        RxtocW::new(self, 0)
    }
    #[doc = "Bit 7 - RXTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxtoen(&mut self) -> RxtoenW<UsartUsrtprSpec> {
        RxtoenW::new(self, 7)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TgW<UsartUsrtprSpec> {
        TgW::new(self, 8)
    }
}
#[doc = "USART_USRTPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrtprSpec;
impl crate::RegisterSpec for UsartUsrtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrtpr::R`](R) reader structure"]
impl crate::Readable for UsartUsrtprSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrtpr::W`](W) writer structure"]
impl crate::Writable for UsartUsrtprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRTPR to value 0"]
impl crate::Resettable for UsartUsrtprSpec {
    const RESET_VALUE: u32 = 0;
}
