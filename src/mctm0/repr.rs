#[doc = "Register `REPR` reader"]
pub type R = crate::R<ReprSpec>;
#[doc = "Register `REPR` writer"]
pub type W = crate::W<ReprSpec>;
#[doc = "Field `REPV` reader - REPV"]
pub type RepvR = crate::FieldReader;
#[doc = "Field `REPV` writer - REPV"]
pub type RepvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    pub fn repv(&self) -> RepvR {
        RepvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    pub fn repv(&mut self) -> RepvW<'_, ReprSpec> {
        RepvW::new(self, 0)
    }
}
#[doc = "REPR\n\nYou can [`read`](crate::Reg::read) this register and get [`repr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReprSpec;
impl crate::RegisterSpec for ReprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repr::R`](R) reader structure"]
impl crate::Readable for ReprSpec {}
#[doc = "`write(|w| ..)` method takes [`repr::W`](W) writer structure"]
impl crate::Writable for ReprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPR to value 0"]
impl crate::Resettable for ReprSpec {}
