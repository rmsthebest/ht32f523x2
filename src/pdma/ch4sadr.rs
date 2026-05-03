#[doc = "Register `CH4SADR` reader"]
pub type R = crate::R<Ch4sadrSpec>;
#[doc = "Register `CH4SADR` writer"]
pub type W = crate::W<Ch4sadrSpec>;
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
    pub fn sadr(&mut self) -> SadrW<'_, Ch4sadrSpec> {
        SadrW::new(self, 0)
    }
}
#[doc = "CH4SADR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4sadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4sadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4sadrSpec;
impl crate::RegisterSpec for Ch4sadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4sadr::R`](R) reader structure"]
impl crate::Readable for Ch4sadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4sadr::W`](W) writer structure"]
impl crate::Writable for Ch4sadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4SADR to value 0"]
impl crate::Resettable for Ch4sadrSpec {}
