#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - TICKINT"]
pub type TickintR = crate::BitReader;
#[doc = "Field `TICKINT` writer - TICKINT"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - CLKSOURCE"]
pub type ClksourceR = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - CLKSOURCE"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTFLAG` reader - COUNTFLAG"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - COUNTFLAG"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TICKINT"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKSOURCE"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - COUNTFLAG"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - TICKINT"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TickintW<'_, CsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - CLKSOURCE"]
    #[inline(always)]
    pub fn clksource(&mut self) -> ClksourceW<'_, CsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - COUNTFLAG"]
    #[inline(always)]
    pub fn countflag(&mut self) -> CountflagW<'_, CsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
