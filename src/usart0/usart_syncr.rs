#[doc = "Register `USART_SYNCR` reader"]
pub type R = crate::R<UsartSyncrSpec>;
#[doc = "Register `USART_SYNCR` writer"]
pub type W = crate::W<UsartSyncrSpec>;
#[doc = "Field `CLKEN` reader - CLKEN"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - CLKEN"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPS` reader - CPS"]
pub type CpsR = crate::BitReader;
#[doc = "Field `CPS` writer - CPS"]
pub type CpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPO` reader - CPO"]
pub type CpoR = crate::BitReader;
#[doc = "Field `CPO` writer - CPO"]
pub type CpoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    pub fn cps(&self) -> CpsR {
        CpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    pub fn cpo(&self) -> CpoR {
        CpoR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<UsartSyncrSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 2 - CPS"]
    #[inline(always)]
    #[must_use]
    pub fn cps(&mut self) -> CpsW<UsartSyncrSpec> {
        CpsW::new(self, 2)
    }
    #[doc = "Bit 3 - CPO"]
    #[inline(always)]
    #[must_use]
    pub fn cpo(&mut self) -> CpoW<UsartSyncrSpec> {
        CpoW::new(self, 3)
    }
}
#[doc = "USART_SYNCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_syncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_syncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartSyncrSpec;
impl crate::RegisterSpec for UsartSyncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_syncr::R`](R) reader structure"]
impl crate::Readable for UsartSyncrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_syncr::W`](W) writer structure"]
impl crate::Writable for UsartSyncrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_SYNCR to value 0"]
impl crate::Resettable for UsartSyncrSpec {
    const RESET_VALUE: u32 = 0;
}
