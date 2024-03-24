#[doc = "Register `CRR` reader"]
pub type R = crate::R<CrrSpec>;
#[doc = "Register `CRR` writer"]
pub type W = crate::W<CrrSpec>;
#[doc = "Field `CRV` reader - CRV"]
pub type CrvR = crate::FieldReader<u16>;
#[doc = "Field `CRV` writer - CRV"]
pub type CrvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    pub fn crv(&self) -> CrvR {
        CrvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    #[must_use]
    pub fn crv(&mut self) -> CrvW<CrrSpec> {
        CrvW::new(self, 0)
    }
}
#[doc = "CRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrrSpec;
impl crate::RegisterSpec for CrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crr::R`](R) reader structure"]
impl crate::Readable for CrrSpec {}
#[doc = "`write(|w| ..)` method takes [`crr::W`](W) writer structure"]
impl crate::Writable for CrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRR to value 0"]
impl crate::Resettable for CrrSpec {
    const RESET_VALUE: u32 = 0;
}
