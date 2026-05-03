#[doc = "Register `PWRCU_BAKTEST` reader"]
pub type R = crate::R<PwrcuBaktestSpec>;
#[doc = "Register `PWRCU_BAKTEST` writer"]
pub type W = crate::W<PwrcuBaktestSpec>;
#[doc = "Field `BAKTEST` reader - BAKTEST"]
pub type BaktestR = crate::FieldReader;
#[doc = "Field `BAKTEST` writer - BAKTEST"]
pub type BaktestW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    pub fn baktest(&self) -> BaktestR {
        BaktestR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    pub fn baktest(&mut self) -> BaktestW<'_, PwrcuBaktestSpec> {
        BaktestW::new(self, 0)
    }
}
#[doc = "PWRCU_BAKTEST\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcu_baktest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcu_baktest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBaktestSpec;
impl crate::RegisterSpec for PwrcuBaktestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_baktest::R`](R) reader structure"]
impl crate::Readable for PwrcuBaktestSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_baktest::W`](W) writer structure"]
impl crate::Writable for PwrcuBaktestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCU_BAKTEST to value 0"]
impl crate::Resettable for PwrcuBaktestSpec {}
