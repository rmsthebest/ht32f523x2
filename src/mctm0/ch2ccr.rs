#[doc = "Register `CH2CCR` reader"]
pub type R = crate::R<Ch2ccrSpec>;
#[doc = "Register `CH2CCR` writer"]
pub type W = crate::W<Ch2ccrSpec>;
#[doc = "Field `CH2CCV` reader - CH2CCV"]
pub type Ch2ccvR = crate::FieldReader<u16>;
#[doc = "Field `CH2CCV` writer - CH2CCV"]
pub type Ch2ccvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&self) -> Ch2ccvR {
        Ch2ccvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2CCV"]
    #[inline(always)]
    pub fn ch2ccv(&mut self) -> Ch2ccvW<'_, Ch2ccrSpec> {
        Ch2ccvW::new(self, 0)
    }
}
#[doc = "CH2CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ccrSpec;
impl crate::RegisterSpec for Ch2ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2ccr::R`](R) reader structure"]
impl crate::Readable for Ch2ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2ccr::W`](W) writer structure"]
impl crate::Writable for Ch2ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2CCR to value 0"]
impl crate::Resettable for Ch2ccrSpec {}
