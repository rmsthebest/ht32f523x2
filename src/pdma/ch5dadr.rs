#[doc = "Register `CH5DADR` reader"]
pub type R = crate::R<Ch5dadrSpec>;
#[doc = "Register `CH5DADR` writer"]
pub type W = crate::W<Ch5dadrSpec>;
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
    pub fn dadr(&mut self) -> DadrW<'_, Ch5dadrSpec> {
        DadrW::new(self, 0)
    }
}
#[doc = "CH5DADR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5dadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5dadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5dadrSpec;
impl crate::RegisterSpec for Ch5dadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5dadr::R`](R) reader structure"]
impl crate::Readable for Ch5dadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5dadr::W`](W) writer structure"]
impl crate::Writable for Ch5dadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5DADR to value 0"]
impl crate::Resettable for Ch5dadrSpec {}
