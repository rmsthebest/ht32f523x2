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
    #[must_use]
    pub fn tadb(&mut self) -> TadbW<TadrSpec> {
        TadbW::new(self, 0)
    }
}
#[doc = "TADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TadrSpec;
impl crate::RegisterSpec for TadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tadr::R`](R) reader structure"]
impl crate::Readable for TadrSpec {}
#[doc = "`write(|w| ..)` method takes [`tadr::W`](W) writer structure"]
impl crate::Writable for TadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TADR to value 0"]
impl crate::Resettable for TadrSpec {
    const RESET_VALUE: u32 = 0;
}
