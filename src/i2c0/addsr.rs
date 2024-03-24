#[doc = "Register `ADDSR` reader"]
pub type R = crate::R<AddsrSpec>;
#[doc = "Register `ADDSR` writer"]
pub type W = crate::W<AddsrSpec>;
#[doc = "Field `ADDSR` reader - ADDSR"]
pub type AddsrR = crate::FieldReader<u16>;
#[doc = "Field `ADDSR` writer - ADDSR"]
pub type AddsrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    pub fn addsr(&self) -> AddsrR {
        AddsrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    #[must_use]
    pub fn addsr(&mut self) -> AddsrW<AddsrSpec> {
        AddsrW::new(self, 0)
    }
}
#[doc = "ADDSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddsrSpec;
impl crate::RegisterSpec for AddsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addsr::R`](R) reader structure"]
impl crate::Readable for AddsrSpec {}
#[doc = "`write(|w| ..)` method takes [`addsr::W`](W) writer structure"]
impl crate::Writable for AddsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDSR to value 0"]
impl crate::Resettable for AddsrSpec {
    const RESET_VALUE: u32 = 0;
}
