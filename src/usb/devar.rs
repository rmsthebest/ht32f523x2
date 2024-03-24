#[doc = "Register `DEVAR` reader"]
pub type R = crate::R<DevarSpec>;
#[doc = "Register `DEVAR` writer"]
pub type W = crate::W<DevarSpec>;
#[doc = "Field `DEVA` reader - DEVA"]
pub type DevaR = crate::FieldReader;
#[doc = "Field `DEVA` writer - DEVA"]
pub type DevaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    pub fn deva(&self) -> DevaR {
        DevaR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    #[must_use]
    pub fn deva(&mut self) -> DevaW<DevarSpec> {
        DevaW::new(self, 0)
    }
}
#[doc = "DEVAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevarSpec;
impl crate::RegisterSpec for DevarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devar::R`](R) reader structure"]
impl crate::Readable for DevarSpec {}
#[doc = "`write(|w| ..)` method takes [`devar::W`](W) writer structure"]
impl crate::Writable for DevarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVAR to value 0"]
impl crate::Resettable for DevarSpec {
    const RESET_VALUE: u32 = 0;
}
