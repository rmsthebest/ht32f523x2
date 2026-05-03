#[doc = "Register `GTR` reader"]
pub type R = crate::R<GtrSpec>;
#[doc = "Register `GTR` writer"]
pub type W = crate::W<GtrSpec>;
#[doc = "Field `GT` reader - GT"]
pub type GtR = crate::FieldReader<u16>;
#[doc = "Field `GT` writer - GT"]
pub type GtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    pub fn gt(&self) -> GtR {
        GtR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - GT"]
    #[inline(always)]
    pub fn gt(&mut self) -> GtW<'_, GtrSpec> {
        GtW::new(self, 0)
    }
}
#[doc = "GTR\n\nYou can [`read`](crate::Reg::read) this register and get [`gtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtrSpec;
impl crate::RegisterSpec for GtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtr::R`](R) reader structure"]
impl crate::Readable for GtrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtr::W`](W) writer structure"]
impl crate::Writable for GtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTR to value 0"]
impl crate::Resettable for GtrSpec {}
