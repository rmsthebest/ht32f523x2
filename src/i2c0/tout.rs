#[doc = "Register `TOUT` reader"]
pub type R = crate::R<ToutSpec>;
#[doc = "Register `TOUT` writer"]
pub type W = crate::W<ToutSpec>;
#[doc = "Field `TOUT` reader - TOUT"]
pub type ToutR = crate::FieldReader<u16>;
#[doc = "Field `TOUT` writer - TOUT"]
pub type ToutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PSC` reader - PSC"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - PSC"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<ToutSpec> {
        ToutW::new(self, 0)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<ToutSpec> {
        PscW::new(self, 16)
    }
}
#[doc = "TOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToutSpec;
impl crate::RegisterSpec for ToutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tout::R`](R) reader structure"]
impl crate::Readable for ToutSpec {}
#[doc = "`write(|w| ..)` method takes [`tout::W`](W) writer structure"]
impl crate::Writable for ToutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUT to value 0"]
impl crate::Resettable for ToutSpec {
    const RESET_VALUE: u32 = 0;
}
