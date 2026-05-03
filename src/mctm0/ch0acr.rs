#[doc = "Register `CH0ACR` reader"]
pub type R = crate::R<Ch0acrSpec>;
#[doc = "Register `CH0ACR` writer"]
pub type W = crate::W<Ch0acrSpec>;
#[doc = "Field `CH0ACV` reader - CH0ACV"]
pub type Ch0acvR = crate::FieldReader<u16>;
#[doc = "Field `CH0ACV` writer - CH0ACV"]
pub type Ch0acvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    pub fn ch0acv(&self) -> Ch0acvR {
        Ch0acvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0ACV"]
    #[inline(always)]
    pub fn ch0acv(&mut self) -> Ch0acvW<'_, Ch0acrSpec> {
        Ch0acvW::new(self, 0)
    }
}
#[doc = "CH0ACR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0acrSpec;
impl crate::RegisterSpec for Ch0acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0acr::R`](R) reader structure"]
impl crate::Readable for Ch0acrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0acr::W`](W) writer structure"]
impl crate::Writable for Ch0acrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0ACR to value 0"]
impl crate::Resettable for Ch0acrSpec {}
