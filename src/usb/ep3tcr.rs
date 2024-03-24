#[doc = "Register `EP3TCR` reader"]
pub type R = crate::R<Ep3tcrSpec>;
#[doc = "Register `EP3TCR` writer"]
pub type W = crate::W<Ep3tcrSpec>;
#[doc = "Field `TCNT` reader - TCNT"]
pub type TcntR = crate::FieldReader<u16>;
#[doc = "Field `TCNT` writer - TCNT"]
pub type TcntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    pub fn tcnt(&self) -> TcntR {
        TcntR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt(&mut self) -> TcntW<Ep3tcrSpec> {
        TcntW::new(self, 0)
    }
}
#[doc = "EP3TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep3tcrSpec;
impl crate::RegisterSpec for Ep3tcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep3tcr::R`](R) reader structure"]
impl crate::Readable for Ep3tcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep3tcr::W`](W) writer structure"]
impl crate::Writable for Ep3tcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP3TCR to value 0"]
impl crate::Resettable for Ep3tcrSpec {
    const RESET_VALUE: u32 = 0;
}
