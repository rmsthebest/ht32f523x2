#[doc = "Register `PWRCU_BAKREG2` reader"]
pub type R = crate::R<PwrcuBakreg2Spec>;
#[doc = "Register `PWRCU_BAKREG2` writer"]
pub type W = crate::W<PwrcuBakreg2Spec>;
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
    pub fn bakreg(&mut self) -> BakregW<'_, PwrcuBakreg2Spec> {
        BakregW::new(self, 0)
    }
}
#[doc = "PWRCU_BAKREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcu_bakreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcu_bakreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBakreg2Spec;
impl crate::RegisterSpec for PwrcuBakreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_bakreg2::R`](R) reader structure"]
impl crate::Readable for PwrcuBakreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_bakreg2::W`](W) writer structure"]
impl crate::Writable for PwrcuBakreg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCU_BAKREG2 to value 0"]
impl crate::Resettable for PwrcuBakreg2Spec {}
