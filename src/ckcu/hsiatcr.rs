#[doc = "Register `HSIATCR` reader"]
pub type R = crate::R<HsiatcrSpec>;
#[doc = "Register `HSIATCR` writer"]
pub type W = crate::W<HsiatcrSpec>;
#[doc = "Field `ATCNT` reader - ATCNT"]
pub type AtcntR = crate::FieldReader<u16>;
#[doc = "Field `ATCNT` writer - ATCNT"]
pub type AtcntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    pub fn atcnt(&self) -> AtcntR {
        AtcntR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    pub fn atcnt(&mut self) -> AtcntW<'_, HsiatcrSpec> {
        AtcntW::new(self, 0)
    }
}
#[doc = "HSIATCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hsiatcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsiatcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsiatcrSpec;
impl crate::RegisterSpec for HsiatcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsiatcr::R`](R) reader structure"]
impl crate::Readable for HsiatcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsiatcr::W`](W) writer structure"]
impl crate::Writable for HsiatcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSIATCR to value 0"]
impl crate::Resettable for HsiatcrSpec {}
