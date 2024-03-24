#[doc = "Register `MR1` reader"]
pub type R = crate::R<Mr1Spec>;
#[doc = "Register `MR1` writer"]
pub type W = crate::W<Mr1Spec>;
#[doc = "Field `WDTD` reader - WDTD"]
pub type WdtdR = crate::FieldReader<u16>;
#[doc = "Field `WDTD` writer - WDTD"]
pub type WdtdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WPSC` reader - WPSC"]
pub type WpscR = crate::FieldReader;
#[doc = "Field `WPSC` writer - WPSC"]
pub type WpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    pub fn wdtd(&self) -> WdtdR {
        WdtdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    pub fn wpsc(&self) -> WpscR {
        WpscR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTD"]
    #[inline(always)]
    #[must_use]
    pub fn wdtd(&mut self) -> WdtdW<Mr1Spec> {
        WdtdW::new(self, 0)
    }
    #[doc = "Bits 12:14 - WPSC"]
    #[inline(always)]
    #[must_use]
    pub fn wpsc(&mut self) -> WpscW<Mr1Spec> {
        WpscW::new(self, 12)
    }
}
#[doc = "MR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mr1Spec;
impl crate::RegisterSpec for Mr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr1::R`](R) reader structure"]
impl crate::Readable for Mr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mr1::W`](W) writer structure"]
impl crate::Writable for Mr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR1 to value 0"]
impl crate::Resettable for Mr1Spec {
    const RESET_VALUE: u32 = 0;
}
