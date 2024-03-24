#[doc = "Register `CH1ACR` reader"]
pub type R = crate::R<Ch1acrSpec>;
#[doc = "Register `CH1ACR` writer"]
pub type W = crate::W<Ch1acrSpec>;
#[doc = "Field `CH1ACV` reader - CH1ACV"]
pub type Ch1acvR = crate::FieldReader<u16>;
#[doc = "Field `CH1ACV` writer - CH1ACV"]
pub type Ch1acvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH1ACV"]
    #[inline(always)]
    pub fn ch1acv(&self) -> Ch1acvR {
        Ch1acvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch1acv(&mut self) -> Ch1acvW<Ch1acrSpec> {
        Ch1acvW::new(self, 0)
    }
}
#[doc = "CH1ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1acrSpec;
impl crate::RegisterSpec for Ch1acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1acr::R`](R) reader structure"]
impl crate::Readable for Ch1acrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1acr::W`](W) writer structure"]
impl crate::Writable for Ch1acrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1ACR to value 0"]
impl crate::Resettable for Ch1acrSpec {
    const RESET_VALUE: u32 = 0;
}
