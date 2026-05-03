#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RxdrSpec>;
#[doc = "Register `RXDR` writer"]
pub type W = crate::W<RxdrSpec>;
#[doc = "Field `RXDR` reader - RXDR"]
pub type RxdrR = crate::FieldReader<u32>;
#[doc = "Field `RXDR` writer - RXDR"]
pub type RxdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RxdrW<'_, RxdrSpec> {
        RxdrW::new(self, 0)
    }
}
#[doc = "RXDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdrSpec;
impl crate::RegisterSpec for RxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdr::W`](W) writer structure"]
impl crate::Writable for RxdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RxdrSpec {}
