#[doc = "Register `EP2TCR` reader"]
pub type R = crate::R<Ep2tcrSpec>;
#[doc = "Register `EP2TCR` writer"]
pub type W = crate::W<Ep2tcrSpec>;
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
    pub fn tcnt(&mut self) -> TcntW<Ep2tcrSpec> {
        TcntW::new(self, 0)
    }
}
#[doc = "EP2TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep2tcrSpec;
impl crate::RegisterSpec for Ep2tcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep2tcr::R`](R) reader structure"]
impl crate::Readable for Ep2tcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep2tcr::W`](W) writer structure"]
impl crate::Writable for Ep2tcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP2TCR to value 0"]
impl crate::Resettable for Ep2tcrSpec {
    const RESET_VALUE: u32 = 0;
}
