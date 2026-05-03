#[doc = "Register `CH1CCR` reader"]
pub type R = crate::R<Ch1ccrSpec>;
#[doc = "Register `CH1CCR` writer"]
pub type W = crate::W<Ch1ccrSpec>;
#[doc = "Field `CH1CCV` reader - CH1CCV"]
pub type Ch1ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH1CCV` writer - CH1CCV"]
pub type Ch1ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    pub fn ch1ccv(&self) -> Ch1ccvR {
        Ch1ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1CCV"]
    #[inline(always)]
    pub fn ch1ccv(&mut self) -> Ch1ccvW<'_, Ch1ccrSpec> {
        Ch1ccvW::new(self, 0)
    }
}
#[doc = "CH1CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ccrSpec;
impl crate::RegisterSpec for Ch1ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccr::R`](R) reader structure"]
impl crate::Readable for Ch1ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ccr::W`](W) writer structure"]
impl crate::Writable for Ch1ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CCR to value 0"]
impl crate::Resettable for Ch1ccrSpec {}
