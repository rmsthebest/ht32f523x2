#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<StrSpec>;
#[doc = "Field `ADST` reader - ADST"]
pub type AdstR = crate::FieldReader;
#[doc = "Field `ADST` writer - ADST"]
pub type AdstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    pub fn adst(&self) -> AdstR {
        AdstR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    #[must_use]
    pub fn adst(&mut self) -> AdstW<StrSpec> {
        AdstW::new(self, 0)
    }
}
#[doc = "STR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`str::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for StrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for StrSpec {
    const RESET_VALUE: u32 = 0;
}
