#[doc = "Register `CH2DADR` reader"]
pub type R = crate::R<Ch2dadrSpec>;
#[doc = "Register `CH2DADR` writer"]
pub type W = crate::W<Ch2dadrSpec>;
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
    pub fn dadr(&mut self) -> DadrW<'_, Ch2dadrSpec> {
        DadrW::new(self, 0)
    }
}
#[doc = "CH2DADR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2dadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2dadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2dadrSpec;
impl crate::RegisterSpec for Ch2dadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2dadr::R`](R) reader structure"]
impl crate::Readable for Ch2dadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2dadr::W`](W) writer structure"]
impl crate::Writable for Ch2dadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2DADR to value 0"]
impl crate::Resettable for Ch2dadrSpec {}
