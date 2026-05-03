#[doc = "Register `RTR` reader"]
pub type R = crate::R<RtrSpec>;
#[doc = "Register `RTR` writer"]
pub type W = crate::W<RtrSpec>;
#[doc = "Field `RDSETUP` reader - RDSETUP"]
pub type RdsetupR = crate::FieldReader;
#[doc = "Field `RDSETUP` writer - RDSETUP"]
pub type RdsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDSTRB` reader - RDSTRB"]
pub type RdstrbR = crate::FieldReader;
#[doc = "Field `RDSTRB` writer - RDSTRB"]
pub type RdstrbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RDHOLD` reader - RDHOLD"]
pub type RdholdR = crate::FieldReader;
#[doc = "Field `RDHOLD` writer - RDHOLD"]
pub type RdholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RdsetupR {
        RdsetupR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RdstrbR {
        RdstrbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&self) -> RdholdR {
        RdholdR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RdsetupW<'_, RtrSpec> {
        RdsetupW::new(self, 0)
    }
    #[doc = "Bits 8:12 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RdstrbW<'_, RtrSpec> {
        RdstrbW::new(self, 8)
    }
    #[doc = "Bits 16:18 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RdholdW<'_, RtrSpec> {
        RdholdW::new(self, 16)
    }
}
#[doc = "RTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtrSpec;
impl crate::RegisterSpec for RtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtr::R`](R) reader structure"]
impl crate::Readable for RtrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtr::W`](W) writer structure"]
impl crate::Writable for RtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTR to value 0"]
impl crate::Resettable for RtrSpec {}
