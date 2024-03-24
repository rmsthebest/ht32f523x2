#[doc = "Register `CH0DADR` reader"]
pub type R = crate::R<Ch0dadrSpec>;
#[doc = "Register `CH0DADR` writer"]
pub type W = crate::W<Ch0dadrSpec>;
#[doc = "Field `DADR` reader - DADR"]
pub type DadrR = crate::FieldReader<u32>;
#[doc = "Field `DADR` writer - DADR"]
pub type DadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    pub fn dadr(&self) -> DadrR {
        DadrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DADR"]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DadrW<Ch0dadrSpec> {
        DadrW::new(self, 0)
    }
}
#[doc = "CH0DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0dadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0dadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0dadrSpec;
impl crate::RegisterSpec for Ch0dadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0dadr::R`](R) reader structure"]
impl crate::Readable for Ch0dadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0dadr::W`](W) writer structure"]
impl crate::Writable for Ch0dadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0DADR to value 0"]
impl crate::Resettable for Ch0dadrSpec {
    const RESET_VALUE: u32 = 0;
}
