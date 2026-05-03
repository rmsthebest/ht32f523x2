#[doc = "Register `EP4TCR` reader"]
pub type R = crate::R<Ep4tcrSpec>;
#[doc = "Register `EP4TCR` writer"]
pub type W = crate::W<Ep4tcrSpec>;
#[doc = "Field `TCNT0` reader - TCNT0"]
pub type Tcnt0R = crate::FieldReader<u16>;
#[doc = "Field `TCNT0` writer - TCNT0"]
pub type Tcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCNT1` reader - TCNT1"]
pub type Tcnt1R = crate::FieldReader<u16>;
#[doc = "Field `TCNT1` writer - TCNT1"]
pub type Tcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    pub fn tcnt0(&self) -> Tcnt0R {
        Tcnt0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    pub fn tcnt1(&self) -> Tcnt1R {
        Tcnt1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    pub fn tcnt0(&mut self) -> Tcnt0W<'_, Ep4tcrSpec> {
        Tcnt0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    pub fn tcnt1(&mut self) -> Tcnt1W<'_, Ep4tcrSpec> {
        Tcnt1W::new(self, 16)
    }
}
#[doc = "EP4TCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4tcrSpec;
impl crate::RegisterSpec for Ep4tcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep4tcr::R`](R) reader structure"]
impl crate::Readable for Ep4tcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep4tcr::W`](W) writer structure"]
impl crate::Writable for Ep4tcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP4TCR to value 0"]
impl crate::Resettable for Ep4tcrSpec {}
