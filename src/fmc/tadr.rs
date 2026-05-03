#[doc = "Register `TADR` reader"]
pub type R = crate::R<TadrSpec>;
#[doc = "Register `TADR` writer"]
pub type W = crate::W<TadrSpec>;
#[doc = "Field `TADB` reader - TADB"]
pub type TadbR = crate::FieldReader<u32>;
#[doc = "Field `TADB` writer - TADB"]
pub type TadbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TADB"]
    #[inline(always)]
    pub fn tadb(&self) -> TadbR {
        TadbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TADB"]
    #[inline(always)]
    pub fn tadb(&mut self) -> TadbW<'_, TadrSpec> {
        TadbW::new(self, 0)
    }
}
#[doc = "TADR\n\nYou can [`read`](crate::Reg::read) this register and get [`tadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TadrSpec;
impl crate::RegisterSpec for TadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tadr::R`](R) reader structure"]
impl crate::Readable for TadrSpec {}
#[doc = "`write(|w| ..)` method takes [`tadr::W`](W) writer structure"]
impl crate::Writable for TadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TADR to value 0"]
impl crate::Resettable for TadrSpec {}
