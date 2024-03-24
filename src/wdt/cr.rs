#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `WDTRS` reader - WDTRS"]
pub type WdtrsR = crate::BitReader;
#[doc = "Field `WDTRS` writer - WDTRS"]
pub type WdtrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSKEY` reader - RSKEY"]
pub type RskeyR = crate::FieldReader<u16>;
#[doc = "Field `RSKEY` writer - RSKEY"]
pub type RskeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - WDTRS"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WdtrsR {
        WdtrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - RSKEY"]
    #[inline(always)]
    pub fn rskey(&self) -> RskeyR {
        RskeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - WDTRS"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WdtrsW<CrSpec> {
        WdtrsW::new(self, 0)
    }
    #[doc = "Bits 16:31 - RSKEY"]
    #[inline(always)]
    #[must_use]
    pub fn rskey(&mut self) -> RskeyW<CrSpec> {
        RskeyW::new(self, 16)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
