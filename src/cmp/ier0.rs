#[doc = "Register `IER0` reader"]
pub type R = crate::R<Ier0Spec>;
#[doc = "Register `IER0` writer"]
pub type W = crate::W<Ier0Spec>;
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
    pub fn cmpfien(&mut self) -> CmpfienW<Ier0Spec> {
        CmpfienW::new(self, 0)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmprien(&mut self) -> CmprienW<Ier0Spec> {
        CmprienW::new(self, 1)
    }
}
#[doc = "IER0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier0Spec;
impl crate::RegisterSpec for Ier0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier0::R`](R) reader structure"]
impl crate::Readable for Ier0Spec {}
#[doc = "`write(|w| ..)` method takes [`ier0::W`](W) writer structure"]
impl crate::Writable for Ier0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER0 to value 0"]
impl crate::Resettable for Ier0Spec {
    const RESET_VALUE: u32 = 0;
}
