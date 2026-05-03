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
    pub fn ch2acv(&mut self) -> Ch2acvW<'_, Ch2acrSpec> {
        Ch2acvW::new(self, 0)
    }
}
#[doc = "CH2ACR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2acrSpec;
impl crate::RegisterSpec for Ch2acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2acr::R`](R) reader structure"]
impl crate::Readable for Ch2acrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2acr::W`](W) writer structure"]
impl crate::Writable for Ch2acrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2ACR to value 0"]
impl crate::Resettable for Ch2acrSpec {}
