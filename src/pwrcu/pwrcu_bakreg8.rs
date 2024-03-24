#[doc = "Register `PWRCU_BAKREG8` reader"]
pub type R = crate::R<PwrcuBakreg8Spec>;
#[doc = "Register `PWRCU_BAKREG8` writer"]
pub type W = crate::W<PwrcuBakreg8Spec>;
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
    #[must_use]
    pub fn bakreg(&mut self) -> BakregW<PwrcuBakreg8Spec> {
        BakregW::new(self, 0)
    }
}
#[doc = "PWRCU_BAKREG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBakreg8Spec;
impl crate::RegisterSpec for PwrcuBakreg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_bakreg8::R`](R) reader structure"]
impl crate::Readable for PwrcuBakreg8Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_bakreg8::W`](W) writer structure"]
impl crate::Writable for PwrcuBakreg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCU_BAKREG8 to value 0"]
impl crate::Resettable for PwrcuBakreg8Spec {
    const RESET_VALUE: u32 = 0;
}
