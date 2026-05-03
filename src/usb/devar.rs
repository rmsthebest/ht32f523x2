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
    pub fn deva(&mut self) -> DevaW<'_, DevarSpec> {
        DevaW::new(self, 0)
    }
}
#[doc = "DEVAR\n\nYou can [`read`](crate::Reg::read) this register and get [`devar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevarSpec;
impl crate::RegisterSpec for DevarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devar::R`](R) reader structure"]
impl crate::Readable for DevarSpec {}
#[doc = "`write(|w| ..)` method takes [`devar::W`](W) writer structure"]
impl crate::Writable for DevarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVAR to value 0"]
impl crate::Resettable for DevarSpec {}
