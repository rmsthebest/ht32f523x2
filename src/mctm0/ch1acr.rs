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
    pub fn ch1acv(&mut self) -> Ch1acvW<'_, Ch1acrSpec> {
        Ch1acvW::new(self, 0)
    }
}
#[doc = "CH1ACR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1acrSpec;
impl crate::RegisterSpec for Ch1acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1acr::R`](R) reader structure"]
impl crate::Readable for Ch1acrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1acr::W`](W) writer structure"]
impl crate::Writable for Ch1acrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1ACR to value 0"]
impl crate::Resettable for Ch1acrSpec {}
