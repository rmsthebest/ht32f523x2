#[doc = "Register `CH2ACR` reader"]
pub type R = crate::R<Ch2acrSpec>;
#[doc = "Register `CH2ACR` writer"]
pub type W = crate::W<Ch2acrSpec>;
#[doc = "Field `CH2ACV` reader - CH2ACV"]
pub type Ch2acvR = crate::FieldReader<u16>;
#[doc = "Field `CH2ACV` writer - CH2ACV"]
pub type Ch2acvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    pub fn ch2acv(&self) -> Ch2acvR {
        Ch2acvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    #[must_use]
    pub fn ch2acv(&mut self) -> Ch2acvW<Ch2acrSpec> {
        Ch2acvW::new(self, 0)
    }
}
#[doc = "CH2ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2acrSpec;
impl crate::RegisterSpec for Ch2acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2acr::R`](R) reader structure"]
impl crate::Readable for Ch2acrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2acr::W`](W) writer structure"]
impl crate::Writable for Ch2acrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2ACR to value 0"]
impl crate::Resettable for Ch2acrSpec {
    const RESET_VALUE: u32 = 0;
}
