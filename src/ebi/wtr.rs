#[doc = "Register `WTR` reader"]
pub type R = crate::R<WtrSpec>;
#[doc = "Register `WTR` writer"]
pub type W = crate::W<WtrSpec>;
#[doc = "Field `WRSETUP` reader - WRSETUP"]
pub type WrsetupR = crate::FieldReader;
#[doc = "Field `WRSETUP` writer - WRSETUP"]
pub type WrsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRSTRB` reader - WRSTRB"]
pub type WrstrbR = crate::FieldReader;
#[doc = "Field `WRSTRB` writer - WRSTRB"]
pub type WrstrbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRHOLD` reader - WRHOLD"]
pub type WrholdR = crate::FieldReader;
#[doc = "Field `WRHOLD` writer - WRHOLD"]
pub type WrholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - WRSETUP"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WrsetupR {
        WrsetupR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - WRSTRB"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WrstrbR {
        WrstrbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - WRHOLD"]
    #[inline(always)]
    pub fn wrhold(&self) -> WrholdR {
        WrholdR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WRSETUP"]
    #[inline(always)]
    #[must_use]
    pub fn wrsetup(&mut self) -> WrsetupW<WtrSpec> {
        WrsetupW::new(self, 0)
    }
    #[doc = "Bits 8:12 - WRSTRB"]
    #[inline(always)]
    #[must_use]
    pub fn wrstrb(&mut self) -> WrstrbW<WtrSpec> {
        WrstrbW::new(self, 8)
    }
    #[doc = "Bits 16:18 - WRHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wrhold(&mut self) -> WrholdW<WtrSpec> {
        WrholdW::new(self, 16)
    }
}
#[doc = "WTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtrSpec;
impl crate::RegisterSpec for WtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtr::R`](R) reader structure"]
impl crate::Readable for WtrSpec {}
#[doc = "`write(|w| ..)` method takes [`wtr::W`](W) writer structure"]
impl crate::Writable for WtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTR to value 0"]
impl crate::Resettable for WtrSpec {
    const RESET_VALUE: u32 = 0;
}
