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
    #[must_use]
    pub fn ch1ccv(&mut self) -> Ch1ccvW<Ch1ccrSpec> {
        Ch1ccvW::new(self, 0)
    }
}
#[doc = "CH1CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ccrSpec;
impl crate::RegisterSpec for Ch1ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccr::R`](R) reader structure"]
impl crate::Readable for Ch1ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ccr::W`](W) writer structure"]
impl crate::Writable for Ch1ccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CCR to value 0"]
impl crate::Resettable for Ch1ccrSpec {
    const RESET_VALUE: u32 = 0;
}
