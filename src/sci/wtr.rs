#[doc = "Register `WTR` reader"]
pub type R = crate::R<WtrSpec>;
#[doc = "Register `WTR` writer"]
pub type W = crate::W<WtrSpec>;
#[doc = "Field `WT` reader - WT"]
pub type WtR = crate::FieldReader<u32>;
#[doc = "Field `WT` writer - WT"]
pub type WtW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WtR {
        WtR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&mut self) -> WtW<'_, WtrSpec> {
        WtW::new(self, 0)
    }
}
#[doc = "WTR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtrSpec;
impl crate::RegisterSpec for WtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtr::R`](R) reader structure"]
impl crate::Readable for WtrSpec {}
#[doc = "`write(|w| ..)` method takes [`wtr::W`](W) writer structure"]
impl crate::Writable for WtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WTR to value 0"]
impl crate::Resettable for WtrSpec {}
