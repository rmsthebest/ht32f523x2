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
    #[must_use]
    pub fn gt(&mut self) -> GtW<GtrSpec> {
        GtW::new(self, 0)
    }
}
#[doc = "GTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtrSpec;
impl crate::RegisterSpec for GtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtr::R`](R) reader structure"]
impl crate::Readable for GtrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtr::W`](W) writer structure"]
impl crate::Writable for GtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTR to value 0"]
impl crate::Resettable for GtrSpec {
    const RESET_VALUE: u32 = 0;
}
