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
    #[must_use]
    pub fn atcnt(&mut self) -> AtcntW<HsiatcrSpec> {
        AtcntW::new(self, 0)
    }
}
#[doc = "HSIATCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsiatcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsiatcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsiatcrSpec;
impl crate::RegisterSpec for HsiatcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsiatcr::R`](R) reader structure"]
impl crate::Readable for HsiatcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsiatcr::W`](W) writer structure"]
impl crate::Writable for HsiatcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSIATCR to value 0"]
impl crate::Resettable for HsiatcrSpec {
    const RESET_VALUE: u32 = 0;
}
