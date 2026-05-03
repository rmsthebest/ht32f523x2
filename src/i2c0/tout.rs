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
    pub fn tout(&mut self) -> ToutW<'_, ToutSpec> {
        ToutW::new(self, 0)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, ToutSpec> {
        PscW::new(self, 16)
    }
}
#[doc = "TOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`tout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToutSpec;
impl crate::RegisterSpec for ToutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tout::R`](R) reader structure"]
impl crate::Readable for ToutSpec {}
#[doc = "`write(|w| ..)` method takes [`tout::W`](W) writer structure"]
impl crate::Writable for ToutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUT to value 0"]
impl crate::Resettable for ToutSpec {}
