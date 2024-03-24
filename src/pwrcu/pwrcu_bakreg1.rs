#[doc = "Register `PWRCU_BAKREG1` reader"]
pub type R = crate::R<PwrcuBakreg1Spec>;
#[doc = "Register `PWRCU_BAKREG1` writer"]
pub type W = crate::W<PwrcuBakreg1Spec>;
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
    pub fn bakreg(&mut self) -> BakregW<PwrcuBakreg1Spec> {
        BakregW::new(self, 0)
    }
}
#[doc = "PWRCU_BAKREG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBakreg1Spec;
impl crate::RegisterSpec for PwrcuBakreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_bakreg1::R`](R) reader structure"]
impl crate::Readable for PwrcuBakreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_bakreg1::W`](W) writer structure"]
impl crate::Writable for PwrcuBakreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCU_BAKREG1 to value 0"]
impl crate::Resettable for PwrcuBakreg1Spec {
    const RESET_VALUE: u32 = 0;
}
