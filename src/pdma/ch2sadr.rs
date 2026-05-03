#[doc = "Register `CH2SADR` reader"]
pub type R = crate::R<Ch2sadrSpec>;
#[doc = "Register `CH2SADR` writer"]
pub type W = crate::W<Ch2sadrSpec>;
#[doc = "Field `SADR` reader - SADR"]
pub type SadrR = crate::FieldReader<u32>;
#[doc = "Field `SADR` writer - SADR"]
pub type SadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&self) -> SadrR {
        SadrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SadrW<'_, Ch2sadrSpec> {
        SadrW::new(self, 0)
    }
}
#[doc = "CH2SADR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2sadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2sadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2sadrSpec;
impl crate::RegisterSpec for Ch2sadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2sadr::R`](R) reader structure"]
impl crate::Readable for Ch2sadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2sadr::W`](W) writer structure"]
impl crate::Writable for Ch2sadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2SADR to value 0"]
impl crate::Resettable for Ch2sadrSpec {}
