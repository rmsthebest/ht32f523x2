#[doc = "Register `IER1` reader"]
pub type R = crate::R<Ier1Spec>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<Ier1Spec>;
#[doc = "Field `CMPFIEN` reader - CMPFIEN"]
pub type CmpfienR = crate::BitReader;
#[doc = "Field `CMPFIEN` writer - CMPFIEN"]
pub type CmpfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRIEN` reader - CMPRIEN"]
pub type CmprienR = crate::BitReader;
#[doc = "Field `CMPRIEN` writer - CMPRIEN"]
pub type CmprienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    pub fn cmpfien(&self) -> CmpfienR {
        CmpfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    pub fn cmprien(&self) -> CmprienR {
        CmprienR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfien(&mut self) -> CmpfienW<Ier1Spec> {
        CmpfienW::new(self, 0)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmprien(&mut self) -> CmprienW<Ier1Spec> {
        CmprienW::new(self, 1)
    }
}
#[doc = "IER1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier1Spec;
impl crate::RegisterSpec for Ier1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier1::R`](R) reader structure"]
impl crate::Readable for Ier1Spec {}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for Ier1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for Ier1Spec {
    const RESET_VALUE: u32 = 0;
}
