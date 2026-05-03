#[doc = "Register `PWRCU_BAKREG9` reader"]
pub type R = crate::R<PwrcuBakreg9Spec>;
#[doc = "Register `PWRCU_BAKREG9` writer"]
pub type W = crate::W<PwrcuBakreg9Spec>;
#[doc = "Field `BAKREG` reader - BAKREG"]
pub type BakregR = crate::FieldReader<u32>;
#[doc = "Field `BAKREG` writer - BAKREG"]
pub type BakregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    pub fn bakreg(&self) -> BakregR {
        BakregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    pub fn bakreg(&mut self) -> BakregW<'_, PwrcuBakreg9Spec> {
        BakregW::new(self, 0)
    }
}
#[doc = "PWRCU_BAKREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcu_bakreg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcu_bakreg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBakreg9Spec;
impl crate::RegisterSpec for PwrcuBakreg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_bakreg9::R`](R) reader structure"]
impl crate::Readable for PwrcuBakreg9Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_bakreg9::W`](W) writer structure"]
impl crate::Writable for PwrcuBakreg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCU_BAKREG9 to value 0"]
impl crate::Resettable for PwrcuBakreg9Spec {}
