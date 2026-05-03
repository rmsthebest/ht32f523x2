#[doc = "Register `CH3CCR` reader"]
pub type R = crate::R<Ch3ccrSpec>;
#[doc = "Register `CH3CCR` writer"]
pub type W = crate::W<Ch3ccrSpec>;
#[doc = "Field `CH3CCV` reader - CH3CCV"]
pub type Ch3ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH3CCV` writer - CH3CCV"]
pub type Ch3ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&self) -> Ch3ccvR {
        Ch3ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3CCV"]
    #[inline(always)]
    pub fn ch3ccv(&mut self) -> Ch3ccvW<'_, Ch3ccrSpec> {
        Ch3ccvW::new(self, 0)
    }
}
#[doc = "CH3CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccrSpec;
impl crate::RegisterSpec for Ch3ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccr::R`](R) reader structure"]
impl crate::Readable for Ch3ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ccr::W`](W) writer structure"]
impl crate::Writable for Ch3ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CCR to value 0"]
impl crate::Resettable for Ch3ccrSpec {}
