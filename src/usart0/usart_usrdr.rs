#[doc = "Register `USART_USRDR` reader"]
pub type R = crate::R<UsartUsrdrSpec>;
#[doc = "Register `USART_USRDR` writer"]
pub type W = crate::W<UsartUsrdrSpec>;
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
    pub fn db(&mut self) -> DbW<UsartUsrdrSpec> {
        DbW::new(self, 0)
    }
}
#[doc = "USART_USRDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartUsrdrSpec;
impl crate::RegisterSpec for UsartUsrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_usrdr::R`](R) reader structure"]
impl crate::Readable for UsartUsrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_usrdr::W`](W) writer structure"]
impl crate::Writable for UsartUsrdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_USRDR to value 0"]
impl crate::Resettable for UsartUsrdrSpec {
    const RESET_VALUE: u32 = 0;
}
